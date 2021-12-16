from setuptools import setup

setup(
    name='prog',
    version='0.1.0',
    py_modules=['prog.cli', 'prog.commands', 'prog.parse', 'prog.util'],
    install_requires=['Click','pyyaml'],
    package_data={'': ['assets/prog.json', 'assets/prog.yml'],},
    entry_points='''
        [console_scripts]
        prog=prog.cli:cli
    ''',
)
