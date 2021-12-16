"""
"   main.py
"   Brian Reece
"""

from json import load as json_load
import click, os, yaml

from prog.commands import selectFile, editFile, genFile, fileType 
from prog.parse import parse

CONTEXT_SETTINGS = dict(help_option_names=['-h', '--help'])
VERSION_MESSAGE = 'prog 0.1.2 20211215\n\nBSD 3-Clause License\nCopyright (c) 2021, Brian Reece\nAll rights reserved.\n\nRedistribution and use in source and binary forms, with or without\nmodification, are permitted provided that the conditions listed in the license are met.\n'

path = None

@click.command(context_settings=CONTEXT_SETTINGS, help='A command line utility for centralizing scripted shell commands via a configurable JSON or YAML file')
@click.option('-j', '--json', is_flag=True, default=False, is_eager=True, callback=fileType, help="Use JSON config file")
@click.option('-y', '--yml', is_flag=True, default=False, is_eager=True, callback=fileType, help="Use YAML config file")
@click.option('-g', '--generate', is_flag=True, expose_value=False, callback=genFile, help='Create a config file')
@click.option('-e', '--edit', is_flag=True, expose_value=False, callback=editFile, help='Edit config file')
@click.option('-v', '--verbose', is_flag=True, default=False, help='Show verbose output')
@click.option('-f', '--file', type=click.Path(exists=True), is_eager=True, expose_value=False, callback=selectFile, help='Path to prog file')
@click.version_option('0.1.0', '-V', '--version', message=VERSION_MESSAGE)
@click.argument('commands', type=str, nargs=-1)
@click.pass_context
def cli(ctx, json, yml, verbose, commands):
    if not commands:
        ctx.exit()
    
    conf = {}

    if verbose:
        click.echo(VERSION_MESSAGE)
   
    if not isinstance(ctx.obj, dict):
        ctx.obj = dict()

    if 'path' not in ctx.obj.keys():
        if os.path.exists('./prog.json'):
            ctx.obj['path'] = './prog.json'
        elif os.path.exists('./prog.yml'):
            ctx.obj['path'] = './prog.yml'
        else:
            click.echo('[ERROR] No config file found!')
            ctx.exit()
    
    if verbose:
        click.echo('[CONFIG] Found config file: ' + ctx.obj['path'])

    with open(ctx.obj['path'], 'rt') as f:
        if ctx.obj['path'][-4:] == 'json':
            conf = json_load(f)
        elif ctx.obj['path'][-3:] == 'yml':
            conf = yaml.safe_load(f)
        else:
            click.echo('[ERROR] ' + ctx.obj['path'][-4:] + ' filetype not recognized!')
            ctx.exit()

    if verbose:
        click.echo('[CONFIG] Loaded config: ' + str(conf))

    parse(ctx, verbose, commands, conf)
    ctx.exit()
