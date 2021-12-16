"""
"   parse.py
"   Brian Reece
"""

import click, subprocess
from prog.util import Index

# TODO: Implement support for nested objects and lists
def parse(ctx, verbose, commands, conf):
    def _parser_fail(ctx, cmd, verbose):
        if verbose: click.echo('[ERROR] ' + cmd + ' command failed, halting execution'); click.echo()
        ctx.exit()

    def _parser(ctx, verbose, commands, index, conf):
        if commands[index.val] not in conf.keys():
            click.echo('[ERROR] ' + commands[index.val] + ' command not found in config file!')
            ctx.exit()
        if isinstance(conf[commands[index.val]], str):
            if verbose: click.echo('[STMT] ' + str(conf[commands[index.val]]))
            if subprocess.run(conf[commands[index.val]], shell=True).returncode != 0:
                _parser_fail(ctx, conf[commands[index.val]], verbose)
            index.val += 1
        elif isinstance(conf[commands[index.val]], list):
            if verbose: click.echo('[LIST] ' + str(conf[commands[index.val]]))
            for command in conf[commands[index.val]]:
                if subprocess.run(command, shell=True).returncode != 0:
                    _parser_fail(ctx, command, verbose)
            index.val += 1
        elif isinstance(conf[commands[index.val]], dict):
            if verbose: click.echo('[DICT]' + str(conf[commands[index.val]]))
            index.val += 1
            _parser(ctx, verbose, commands, index, conf[commands[index.val - 1]])

    index = Index(0)
    while index.val < len(commands):
        _parser(ctx, verbose, commands, index, conf)
