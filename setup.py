from setuptools import setup

setup(
    name='prog',
    version='0.1.0',
    py_modules=['prog'],
    install_requires=['Click',],
    package_data={'': ['prog.json'],},
    entry_points='''
        [console_scripts]
        prog=prog:cli
    ''',
)
