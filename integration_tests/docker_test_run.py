#!/usr/bin/env python3
import argparse
import time
import requests
import sys
import subprocess
import textwrap


class Docker:
    def __init__(self, image, run_opts, run_args, port):
        self.image = image
        self.run_opts= run_opts
        self.run_args = run_args
        self.port = port
        self.container = None

    def __enter__(self):
        print("starting container:", self)
        self.container = subprocess.check_output(["docker", "run", "-d", "--rm" ] \
            + self.run_opts + [self.image] + self.run_args, universal_newlines=True).strip()
        try:
            self._wait_for_s3()
            sys.stdout.flush()
        except BaseException:
            self._kill()
            raise

        return self

    def __exit__(self, exc_type, exc_value, traceaback):
        self._kill()

    def _wait_for_s3(self):
        while True:
            time.sleep(1)
            try:
                requests.get("http://localhost:{}".format(self.port))
                break
            except requests.exceptions.ConnectionError:
                print("waiting for container to become ready", self)

        print("container ready, waiting another 5 seconds to ensure everything is set up")
        time.sleep(5)

    def _kill(self):
        if self.container is not None:
            print("terminating docker:", self)
            if subprocess.call(["docker", "kill", self.container]) != 0:
                print("Failed to terminate container.")

    def __str__(self):
        if self.container is not None:
            return "image {!r}, container {!r}".format(self.image, self.container[:12])
        else:
            return "image {!r}".format(self.image)


def parse_args(args):
    epilog = textwrap.dedent("""
        Example:

            * Run docker image ceph/demo.
            * Pass `--env=CEPH_DEMO_ACCESS_KEY=access_key` and `--env=CEPH_DEMO_SECRET_KEY=secret_key` to docker run.
            * Wait for service to become ready by waiting for an HTTP request to succeed. (The container shares the
              host's network.)
            * Once container is ready execute `cargo test --features s3`.

            $ {} --docker-image ceph/demo --run-opt=--env=CEPH_DEMO_ACCESS_KEY=access_key \\
                 --run-opt=--env=CEPH_DEMO_SECRET_KEY=secret_key -- cargo test --features s3
    """).format(sys.argv[0])
    parser = argparse.ArgumentParser(
                description="Start docker image, wait for it do become ready, then execute given command.",
                formatter_class=argparse.RawDescriptionHelpFormatter,
                epilog=epilog)
    parser.add_argument("--docker-image", required=True, dest="docker_images", action="append",
                        help="Name of the docker image to start before executing command. May be given multiple times.")
    parser.add_argument("--run-opt", dest="docker_run_opts", default=[], action="append",
                        help="Option passed on to docker run.")
    parser.add_argument("--run-arg", dest="docker_run_args", default=[], action="append",
                        help="Argument passed on to docker run.")
    parser.add_argument("--port", type=int, default=80,
                        help="Port on which the service is going to be listening. Command is only executed once the "
                             "port (default %(default)s) returns a valid HTTP response.")
    parser.add_argument("command", help="Command to execute.")
    parser.add_argument("args", nargs="*", help="Arguments passed to command. May be given multiple times.")
    return parser.parse_args(args[1:])


def main():
    args = parse_args(sys.argv)
    rc = 0
    for image in args.docker_images:
        with Docker(image=image, run_opts=args.docker_run_opts, run_args=args.docker_run_args, port=args.port) as docker:
            if subprocess.call([args.command] + args.args) != 0:
                if sys.stdout.isatty():
                    print('\033[31m', end='')
                print("ERROR: Docker test failed:", docker)
                if sys.stdout.isatty():
                    print('\033[0m', end='')
                rc = 1

    return rc


if __name__ == "__main__":
    exit(main())
