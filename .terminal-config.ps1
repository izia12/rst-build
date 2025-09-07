# Конфигурация терминала для проекта wasm_3d
# Исправляет проблемы с бесконечными путями

# Очистка переменных окружения
$env:PS1 = $null
$env:PROMPT = $null

# Установка простого промпта
function prompt {
    "PS $(Get-Location)> "
}

# Очистка истории команд
Clear-History

# Установка рабочей директории
Set-Location "c:\workProject\react-3d\wasm_3d"

Write-Host "Терминал настроен правильно. Используйте стандартные команды." -ForegroundColor Green