from setuptools import setup

from getmeta import __version__

with open("README.md", "r") as f:
    long_description = f.read()

setup(
    name = "getmeta",
    version = __version__,
    description = "Not just gold builds anymore!",
    long_description = long_description,
    long_description_content_type = "text/markdown",
    url = "https://github.com/4n6ir/getmeta",
    author = "John Lukach",
    author_email = "hello@4n6ir.com",
    license = "Apache-2.0",
    packages = [
        "getmeta"
    ],
    install_requires = [
        "requests"
    ],
    zip_safe = False,
    entry_points = {
        "console_scripts": [
            "getmeta=getmeta.cli:main"
        ],
    },
    python_requires = ">=3.8",
)