@echo off
REM check if node_modules dir exists
IF NOT EXIST "node_modules" (
    yarn install
)

REM Check if the first argument (%1) is equal to "build"
IF "%1"=="build" (
    yarn tauri build
) ELSE (
    yarn tauri dev
)