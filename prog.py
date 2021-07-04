from json import load
import click, os, pkg_resources

version_message = 'prog 0.1.0 20210703\n\nBSD 3-Clause License\nCopyright (c) 2021, Brian Reece\nAll rights reserved.\n\nRedistribution and use in source and binary forms, with or without\nmodification, are permitted provided that the conditions listed in the license are met.\n'

def generate(ctx, param, value):
    if not param or ctx.resilient_parsing:
        return value
    path = './prog.json'
    buffer = pkg_resources.resource_string(__name__, 'prog.json').decode('utf-8')
    if value:
        path = click.format_filename(value)
    with open(path, 'wt') as f:
        f.write(buffer)
    ctx.exit()

@click.command()
@click.option('-g', '--generate', required=False, type=click.Path(), is_eager=True, callback=generate, help='Generate default prog.json file')
@click.option('-v', '--verbose', is_flag=True, default=False, help='Show verbose output')
@click.option('-f', '--file', required=False, type=click.Path(exists=True), help='Path to prog.json file')
@click.version_option(version='0.1.0', message=version_message)
@click.argument('commands', type=str, nargs=-1)
@click.pass_context
def cli(ctx, generate, verbose, file, commands):
    conf = {}
    path = './prog.json'
    if file:
        path = click.format_filename(file)
    if os.path.exists(path):
        with open(path, 'rt') as f:
            conf = load(f)
    for command in commands:
        cmd = conf.get(command)
        if cmd is not None:
            os.system(cmd)
        else:
            click.echo('No command specified: ' + command)
            ctx.exit()
