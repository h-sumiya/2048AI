ECHO OFF
maturin build --release


for %%i in (.\target\wheels\*.whl) do (
    echo %%i
    pip install %%i --force-reinstall
)

