from json import load
import click, os, pkg_resources

CONTEXT_SETTINGS = dict(help_option_names=['-h', '--help'])
VERSION_MESSAGE = 'prog 0.1.0 20210703\n\nBSD 3-Clause License\nCopyright (c) 2021, Brian Reece\nAll rights reserved.\n\nRedistribution and use in source and binary forms, with or without\nmodification, are permitted provided that the conditions listed in the license are met.\n'

def edit(ctx, param, value):
    if not value or ctx.resilient_parsing:
        return value
    path = './prog.json'
    if value:
        path = click.format_filename(value)
    click.edit(require_save=False, filename=path)
    ctx.exit()

def generate(ctx, param, value):
    if not value or ctx.resilient_parsing:
        return value
    path = './prog.json'
    buffer = pkg_resources.resource_string(__name__, 'prog.json').decode('utf-8')
    if value:
        path = click.format_filename(value)
    with open(path, 'wt') as f:
        f.write(buffer)
    click.edit(require_save=False, filename=path)
    ctx.exit()

@click.command(context_settings=CONTEXT_SETTINGS, help='A command line utility for centralizing scripted shell commands via a configurable JSON file')
@click.option('-g', '--generate', type=click.Path(), is_flag=False, expose_value=False, flag_value='./prog.json', is_eager=True, callback=generate, help='Generate default JSON file')
@click.option('-e', '--edit', type=click.Path(exists=True), is_flag=False, expose_value=False, flag_value='./prog.json', is_eager=True, callback=edit, help='Edit JSON file')
@click.option('-v', '--verbose', is_flag=True, default=False, help='Show verbose output')
@click.option('-f', '--file', required=False, type=click.Path(exists=True), help='Path to JSON file')
@click.version_option('0.1.0', '-V', '--version', message=VERSION_MESSAGE)
@click.argument('commands', type=str, nargs=-1)
@click.pass_context
def cli(ctx, verbose, file, commands):
    if not commands:
        ctx.exit()
    conf = {}
    path = './prog.json'
    if verbose: click.echo(VERSION_MESSAGE)
    if file:
        path = click.format_filename(file)
    if os.path.exists(path):
        if verbose: click.echo('Opening file: ' + path)
        with open(path, 'rt') as f:
            conf = load(f)
    for command in commands:
        if verbose: click.echo(); click.echo('Resolving command: ' + command)
        cmd = conf.get(command)
        if cmd is not None:
            if verbose: click.echo('Executing command: ' + cmd); click.echo()
            os.system(cmd)
        else:
            click.echo('No command specified: ' + command)
            ctx.exit()
