import click, pkg_resources, os

def fileType(ctx, param, value):
    if not value or ctx.resilient_parsing:
        return value

    if not isinstance(ctx.obj, dict):
        ctx.obj = dict()

    if str(param)[-2] == 'l':
        ctx.obj['filetype'] = str(param)[-4:-1]
    else:
        ctx.obj['filetype'] = str(param)[-5:-1]

def selectFile(ctx, param, value):
    if not value or ctx.resilient_parsing:
        return value

    if not isinstance(ctx.obj, dict):
        ctx.obj = dict()

    ctx.obj['path'] = str(click.format_filename(value))
    if ctx.obj['path'][-3:] == 'yml':
        ctx.obj['filetype'] = 'yml'
    elif ctx.obj['path'][-4:] == 'json':
        ctx.obj['filetype'] = 'json'
    else:
        click.echo('[ERROR] Unsupported file type')
        ctx.exit()

def editFile(ctx, param, value):
    if not value or ctx.resilient_parsing:
        return value

    if not isinstance(ctx.obj, dict):
        ctx.obj = dict()
        if os.path.exists('prog.json'):
            ctx.obj['path'] = 'prog.json'
            ctx.obj['filetype'] = 'json'
        elif os.path.exists('prog.yml'):
            ctx.obj['path'] = 'prog.yml'
            ctx.obj['filetype'] = 'yml'
        else:
            click.echo('[ERROR] config file not found')
            ctx.exit()

    
    click.edit(require_save=False, filename=ctx.obj['path'])

def genFile(ctx, param, value):
    if not value or ctx.resilient_parsing:
        return value

    if not isinstance(ctx.obj, dict):
        click.echo('[ERROR] Must specify file type or file name')
        ctx.exit()

    if 'path' not in ctx.obj.keys():
        ctx.obj['path'] = 'prog.' + ctx.obj['filetype']
    elif 'filetype' not in ctx.obj.keys():
        if ctx.obj['path'][-1] == 'l':
            ctx.obj['filetype'] = ctx.obj['path'][-3:]
        else:
            ctx.obj['filetype'] = ctx.obj['path'][-4:]
    
    buffer = pkg_resources.resource_string(__name__, 'assets/prog.' + ctx.obj['filetype']).decode('utf-8')
    with open(ctx.obj['path'], 'wt') as f:
        f.write(buffer)

