import { execFile, spawn } from "node:child_process";
import { randomUUID } from "node:crypto";
import {
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  rename,
  rm,
  writeFile,
} from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { promisify } from "node:util";
const execute = promisify(execFile);
export function detachedProcess(environment = process.env) {
  return (
    process.platform !== "win32" &&
    environment.ASH_DEV_PROCESS_SUPERVISED !== "1"
  );
}
export function isProcessRunning(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    if (error.code === "ESRCH") return false;
    if (error.code === "EPERM") return true;
    throw error;
  }
}
export async function registerRunResource(
  daemon,
  directory = process.env.ASH_SMOKE_RUN_DIRECTORY,
) {
  const path = directory
    ? join(directory, `resource-${randomUUID()}.json`)
    : undefined;
  const resource = {
    daemon: directory
      ? typeof daemon === "function"
        ? daemon()
        : daemon
      : undefined,
  };
  const save = async () => {
    if (!path) return;
    await writeFile(`${path}.tmp`, JSON.stringify(resource));
    await rename(`${path}.tmp`, path);
  };
  await save();
  return {
    process: async (child) => {
      if (!path || !child.pid) return;
      resource.process = (await processes()).get(child.pid);
      await save();
    },
    dispose: async () => {
      if (path)
        await rm(path, {
          force: true,
        });
    },
  };
}
export class ProcessRun {
  children = new Map();
  closing;
  finishGrace;
  owned = new Map();
  registrations = [];
  stopped = false;
  cwd;
  directory;
  graceMs;
  constructor(cwd, directory, graceMs = 45_000) {
    this.cwd = cwd;
    this.directory = directory;
    this.graceMs = graceMs;
  }
  async command(command, args, env, capture = false) {
    if (this.stopped)
      throw new Error(
        "Process run was stopped; refusing to start another command.",
      );
    const child = spawn(command, args, {
      cwd: this.cwd,
      env,
      stdio: capture ? ["ignore", "pipe", "inherit"] : "inherit",
      detached: detachedProcess(env),
      windowsHide: true,
    });
    let output = "";
    child.stdout?.on("data", (data) => {
      output += data.toString();
    });
    const result = new Promise((resolveExit, reject) => {
      child.once("error", reject);
      child.once(capture ? "close" : "exit", (code, signal) =>
        resolveExit({
          code:
            signal === "SIGINT"
              ? 130
              : signal === "SIGTERM"
                ? 143
                : (code ?? 1),
          output,
        }),
      );
    });
    this.children.set(
      child,
      result.then(
        () => undefined,
        () => undefined,
      ),
    );
    await this.rememberTree();
    await this.register(child);
    return await result;
  }
  async own(child) {
    if (this.stopped)
      throw new Error(
        "Process run was stopped; refusing to own another process.",
      );
    let spawnError;
    child.once("error", (error) => {
      spawnError = error;
    });
    this.children.set(
      child,
      child.exitCode !== null || child.signalCode !== null || !child.pid
        ? Promise.resolve()
        : new Promise((resolveExit) => {
            child.once("exit", () => resolveExit());
            child.once("error", () => resolveExit());
          }),
    );
    await this.rememberTree();
    await this.register(child);
    if (spawnError) throw spawnError;
  }
  async register(child) {
    if (process.env.ASH_DEV_PROCESS_SUPERVISED !== "1" || !child.pid) return;
    const resource = await registerRunResource();
    this.registrations.push(resource);
    await resource.process(child);
  }
  stop(signal) {
    if (this.closing) {
      this.finishGrace?.();
      return this.closing;
    }
    this.stopped = true;
    const interruptedAgain = new Promise((resolveGrace) => {
      this.finishGrace = resolveGrace;
    });
    return (this.closing = (async () => {
      const errors = [];
      try {
        await this.rememberTree();
      } catch (error) {
        errors.push(error);
      }
      for (const child of this.children.keys()) {
        if (child.pid && child.exitCode === null && child.signalCode === null)
          child.kill(signal);
      }
      let timeout;
      try {
        timeout = setTimeout(() => this.finishGrace?.(), this.graceMs);
        await Promise.race([
          Promise.all(this.children.values()),
          interruptedAgain,
        ]);
      } finally {
        clearTimeout(timeout);
        this.finishGrace = undefined;
      }
      try {
        await this.rememberTree();
      } catch (error) {
        errors.push(error);
      }
      try {
        await this.cleanup();
      } catch (error) {
        errors.push(error);
      }
      if (errors.length)
        throw new AggregateError(errors, "Process cancellation failed");
    })());
  }
  close() {
    this.stopped = true;
    return (this.closing ??= this.cleanup());
  }
  async resources() {
    if (!this.directory) return [];
    const names = (await readdir(this.directory)).filter(
      (name) => name.startsWith("resource-") && name.endsWith(".json"),
    );
    const resources = [];
    for (const name of names) {
      try {
        resources.push(
          JSON.parse(await readFile(join(this.directory, name), "utf8")),
        );
      } catch (error) {
        if (error.code !== "ENOENT") throw error;
      }
    }
    return resources;
  }
  async rememberTree() {
    const current = await processes();
    const roots = new Set();
    for (const child of this.children.keys()) {
      if (
        child.pid &&
        child.exitCode === null &&
        child.signalCode === null &&
        current.has(child.pid)
      )
        roots.add(child.pid);
    }
    for (const resource of await this.resources()) {
      if (!resource.process) continue;
      const active = current.get(resource.process.pid);
      if (active && active.started !== resource.process.started) continue;
      this.owned.set(resource.process.pid, resource.process);
      roots.add(resource.process.pid);
    }
    for (const identity of this.owned.values()) {
      if (current.get(identity.pid)?.started === identity.started)
        roots.add(identity.pid);
    }
    let changed = true;
    while (changed) {
      changed = false;
      const groups = new Set(
        [...this.owned.values()]
          .filter(
            (identity) =>
              identity.group === identity.pid &&
              (!current.has(identity.pid) ||
                current.get(identity.pid)?.started === identity.started),
          )
          .map((identity) => identity.pid),
      );
      for (const pid of roots) {
        const identity = current.get(pid);
        if (identity?.group === pid) groups.add(pid);
      }
      for (const identity of current.values()) {
        if (
          !roots.has(identity.pid) &&
          (roots.has(identity.parent) ||
            (identity.group !== undefined && groups.has(identity.group)))
        ) {
          roots.add(identity.pid);
          changed = true;
        }
      }
    }
    for (const pid of roots) {
      const identity = current.get(pid);
      if (identity) this.owned.set(pid, identity);
    }
    return current;
  }
  async cleanup() {
    const errors = [];
    let current;
    try {
      current = await this.rememberTree();
    } catch (error) {
      errors.push(error);
    }
    const resources = await this.resources();
    current ??= await processes();
    const terminationErrors = new Map();
    if (process.platform !== "win32") {
      for (const root of this.owned.values()) {
        if (root.group !== root.pid) continue;
        const memberAlive = [...this.owned.values()].some(
          (member) =>
            member.group === root.pid &&
            current.get(member.pid)?.started === member.started,
        );
        if (!memberAlive) continue;
        try {
          process.kill(-root.pid, "SIGKILL");
        } catch (error) {
          if (error.code !== "ESRCH") errors.push(error);
        }
      }
    }
    for (const identity of [...this.owned.values()].reverse()) {
      if (current.get(identity.pid)?.started !== identity.started) continue;
      try {
        if (process.platform === "win32")
          await execute(
            "taskkill",
            ["/PID", String(identity.pid), "/T", "/F"],
            {
              windowsHide: true,
              timeout: 5_000,
            },
          );
        else process.kill(identity.pid, "SIGKILL");
      } catch (error) {
        if (error.code !== "ESRCH") terminationErrors.set(identity.pid, error);
      }
    }
    const deadline = Date.now() + 5_000;
    while (true) {
      const remaining = await processes();
      const alive = [...this.owned.values()].filter(
        (identity) => remaining.get(identity.pid)?.started === identity.started,
      );
      if (!alive.length) break;
      if (Date.now() >= deadline) {
        for (const identity of alive)
          if (terminationErrors.has(identity.pid))
            errors.push(terminationErrors.get(identity.pid));
        errors.push(
          new Error(
            `Owned processes did not exit: ${alive.map((identity) => identity.pid).join(", ")}`,
          ),
        );
        break;
      }
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 50));
    }
    const stops = await Promise.allSettled(
      resources
        .filter((resource) => resource.daemon)
        .map(async (resource) => {
          await execute(resource.daemon.executable, ["stop"], {
            env: {
              ...process.env,
              ASH_HOME: resource.daemon.profile,
            },
            windowsHide: true,
            timeout: 30_000,
          });
        }),
    );
    for (const result of stops)
      if (result.status === "rejected") errors.push(result.reason);
    if (errors.length)
      throw new AggregateError(errors, "Process resource cleanup failed");
    await Promise.all(this.registrations.map((resource) => resource.dispose()));
  }
}
async function processes() {
  if (process.platform === "win32") {
    const environment = Object.fromEntries(
      Object.entries(process.env).filter(
        ([name]) => name.toUpperCase() !== "PSMODULEPATH",
      ),
    );
    const query = execute(
      "powershell.exe",
      [
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        'Get-CimInstance Win32_Process -Property ProcessId,ParentProcessId,CreationDate | Where-Object { $_.CreationDate } | Select-Object ProcessId,ParentProcessId,@{n="Started";e={$_.CreationDate.ToUniversalTime().Ticks.ToString()}} | ConvertTo-Json -Compress',
      ],
      {
        env: environment,
        windowsHide: true,
        timeout: 45_000,
        maxBuffer: 8 * 1024 * 1024,
      },
    );
    query.child.stdin?.end();
    const { stdout } = await query;
    const entries = JSON.parse(stdout);
    return new Map(
      entries
        .filter((entry) => isProcessRunning(entry.ProcessId))
        .map((entry) => [
          entry.ProcessId,
          {
            pid: entry.ProcessId,
            parent: entry.ParentProcessId,
            started: entry.Started,
          },
        ]),
    );
  }
  const { stdout } = await execute(
    "ps",
    ["-axo", "pid=,ppid=,pgid=,stat=,lstart="],
    {
      timeout: 5_000,
      maxBuffer: 8 * 1024 * 1024,
    },
  );
  const entries = new Map();
  for (const line of stdout.split("\n")) {
    const match = line.match(/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\S+)\s+(.+)$/u);
    if (match && !match[4].startsWith("Z"))
      entries.set(Number(match[1]), {
        pid: Number(match[1]),
        parent: Number(match[2]),
        group: Number(match[3]),
        started: match[5].trim(),
      });
  }
  return entries;
}
export async function withProcessRun(cwd, action, options = {}) {
  const inherited = process.env.ASH_SMOKE_RUN_DIRECTORY;
  const directory = inherited ?? (await mkdtemp(join(tmpdir(), "ash-run-")));
  const temporaryDirectory = join(directory, "tmp");
  const run = new ProcessRun(cwd, directory, options.graceMs);
  let interruption;
  const interrupt = (signal) => {
    interruption ??= signal === "SIGINT" ? 130 : 143;
    void run.stop(signal).catch(() => {});
  };
  const onInterrupt = () => interrupt("SIGINT");
  const onTerminate = () => interrupt("SIGTERM");
  process.on("SIGINT", onInterrupt);
  process.on("SIGTERM", onTerminate);
  let actionFailed = false;
  let actionFailure;
  try {
    await mkdir(temporaryDirectory, {
      recursive: true,
    });
    await action(run, {
      ...process.env,
      ASH_SMOKE_RUN_DIRECTORY: directory,
      TMPDIR: temporaryDirectory,
      TMP: temporaryDirectory,
      TEMP: temporaryDirectory,
    });
  } catch (error) {
    if (interruption === undefined) {
      actionFailed = true;
      actionFailure = error;
      throw error;
    }
  } finally {
    try {
      try {
        await run.close();
        if (interruption) process.exitCode = interruption;
      } catch (error) {
        if (actionFailed)
          throw new AggregateError(
            [actionFailure, error],
            "Process action and cleanup failed",
          );
        throw error;
      }
    } finally {
      process.off("SIGINT", onInterrupt);
      process.off("SIGTERM", onTerminate);
      if (!inherited)
        await rm(directory, {
          force: true,
          recursive: true,
          maxRetries: 10,
          retryDelay: 100,
        });
    }
  }
}
