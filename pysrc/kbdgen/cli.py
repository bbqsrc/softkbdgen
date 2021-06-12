import argparse
import yaml
import sys
import os.path
import platform
import logging

from . import __version__, gen
from .base import KbdgenException, Parser, get_logger, UserException

logger = get_logger(__name__)


def parse_args(args):
    def logging_type(string):
        n = {
            "critical": 50,
            "error": 40,
            "warning": 30,
            "info": 20,
            "debug": 10,
            "trace": 5,
        }.get(string, None)

        if n is None:
            raise argparse.ArgumentTypeError("Invalid logging level.")
        return n

    p = argparse.ArgumentParser(prog="kbdgen")

    p.add_argument("--version", action="version", version="%(prog)s " + __version__)
    p.add_argument("--logging", type=logging_type, default=20, help="Logging level")
    p.add_argument("--local", action="store_true", help="local build")
    p.add_argument("--legacy", action="store_true", help='(Windows only)')
    p.add_argument(
        "-K",
        "--key",
        nargs="*",
        dest="cfg_pairs",
        help="Key-value overrides (eg -K target.thing.foo=42)",
    )
    p.add_argument(
        "-D",
        "--dry-run",
        action="store_true",
        help="Don't build, just do requirement validation.",
    )
    p.add_argument(
        "-R",
        "--release",
        action="store_true",
        help="Compile in 'release' mode (where necessary).",
    )
    p.add_argument(
        "-G",
        "--global",
        type=argparse.FileType("r"),
        help="Override the global.yaml file",
    )
    p.add_argument("-r", "--kbd-repo", help="Git repo to generate output from")
    p.add_argument(
        "-b", "--kbd-branch", default="main", help="Git branch (default: main)"
    )
    p.add_argument("--divvunspell-repo")
    p.add_argument("--divvunspell-branch")
    p.add_argument(
        "-t",
        "--target",
        required=True,
        choices=gen.keys(),
        help="Target output.",
    )
    p.add_argument(
        "-o",
        "--output",
        default=".",
        help="Output directory (default: current working directory)",
    )
    p.add_argument("project", help="Keyboard generation bundle (.kbdgen)")
    p.add_argument(
        "-f",
        "--flag",
        nargs="*",
        dest="flags",
        help="Generator-specific flags (for debugging)",
        default=[],
    )
    p.add_argument(
        "-l", "--layout", help="Apply target to specified layout only (EXPERIMENTAL)"
    )
    p.add_argument("--github-username", help="GitHub username for source getting")
    p.add_argument("--github-token", help="GitHub token for source getting")
    p.add_argument("-c", "--command", help="Command to run for a given generators")
    p.add_argument("--ci", action="store_true", help="Continuous integration build")

    return p.parse_args(args)


# def assert_not_inside_mod(output_dir):
#     abs_output = os.path.abspath(output_dir)
#     abs_current = os.path.abspath(os.path.join(__package__, ".."))

#     if abs_output == abs_current:
#         logger.fatal("Your output directory must NOT be the kbdgen module itself!")
#         logger.fatal("Provided output path: '%s'" % abs_output)
#         sys.exit(1)


def enable_verbose_requests_log():
    from http.client import HTTPConnection

    HTTPConnection.debuglevel = 1
    requests_log = logging.getLogger("urllib3")
    # requests_log.setLevel(logging.DEBUG)
    # requests_log.propagate = True


def print_diagnostics():
    logger.debug("Python version: %r" % " ".join(sys.version.split("\n")))
    logger.debug("Platform: %r" % platform.platform())
    logger.debug("Environment:")
    for k, v in os.environ.items():
        logger.debug("  %s = %r" % (k, v))


def run_cli(cli_args):
    args = parse_args(cli_args)
    # logger.setLevel(args.logging)

    print_diagnostics()

    if args.logging == 5:  # logging.TRACE
        enable_verbose_requests_log()

    try:
        project = Parser().parse(args.project, args.cfg_pairs)
        if project is None:
            raise Exception("Project parser returned empty project.")
    except yaml.scanner.ScannerError as e:
        logger.critical(
            "Error parsing project:\n%s %s"
            % (str(e.problem).strip(), str(e.problem_mark).strip())
        )
        return 1
    except Exception as e:
        for arg in e.args:
            logger.critical(str(arg))

        # Short-circuit for user-caused exceptions
        if isinstance(e, UserException):
            return 1

        logger.critical(
            "You should not be seeing this error. Please report this as a bug."
        )
        logger.critical(
            "To receive a more detailed stacktrace, add `--logging trace` to your build command and submit it with your bug report."
        )
        logger.critical("URL: <https://github.com/divvun/kbdgen/issues/>")
        return 1

    generator = gen.get(args.target)

    if generator is None:
        print("Error: '%s' is not a valid target." % args.target, file=sys.stderr)
        print("Valid targets: %s" % ", ".join(gen.keys()), file=sys.stderr)
        return 1

    x = generator(project, dict(args._get_kwargs()))

    # assert_not_inside_mod(x.output_dir)
    try:
        x.generate(x.output_dir)
    except KbdgenException as e:
        logger.error(e)
