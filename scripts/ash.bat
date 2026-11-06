@echo off
setlocal
pushd "%~dp0.." || exit /b 1

if "%~1"=="" goto full

>&2 echo Unknown Ash argument: %~1
popd
exit /b 2

:full
call just ash
set "result=%errorlevel%"
popd
exit /b %result%
