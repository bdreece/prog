from setuptools import setup

long_description = None

with open("README.md", "r") as f:
    long_description = f.read()

setup(
    name='prog',
    version='0.1.2',
    author='Brian Reece',
    author_email='bdreece@mtu.edu',
    description='A tool for centralizing scripted shell commands via a configurable JSON or YAML file',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://github.com/bdreece/prog',
    install_requires=['Click','pyyaml'],
    packages=['prog'],
    package_data={'': ['assets/prog.json', 'assets/prog.yml'],},
    entry_points='''
        [console_scripts]
        prog=prog.cli:cli
        ''',
    python_requires='>=3.6'
)
