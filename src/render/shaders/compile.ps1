if(Get-Command glslangValidator -ErrorAction SilentlyContinue)
{
    Set-Location -Path ".\src\render\shaders"

    Write-Host "Compiling vertex shader..." -ForegroundColor Yellow
    glslangValidator -V ./src/shader.vert -o ./spv/shader.vert.spv
    Write-Host "Completed vertex shader compilation" -ForegroundColor Yellow
    Write-Host "Compiling fragment shader..." -ForegroundColor Yellow
    glslangValidator -V ./src/shader.frag -o ./spv/shader.frag.spv
    Write-Host "Completed fragment shader compilation" -ForegroundColor Yellow
}
else
{
    Write-Error "glslangValidator does not exist"
}