from setuptools import setup

version = {}
with open("datasette_sqlite_regex/version.py") as fp:
    exec(fp.read(), version)

VERSION = version['__version__']

setup(
    name="datasette-sqlite-regex",
    description="",
    long_description="",
    long_description_content_type="text/markdown",
    author="Alex Garcia",
    url="https://github.com/asg017/sqlite-regex",
    project_urls={
        "Issues": "https://github.com/asg017/sqlite-regex/issues",
        "CI": "https://github.com/asg017/sqlite-regex/actions",
        "Changelog": "https://github.com/asg017/sqlite-regex/releases",
    },
    license="MIT License, Apache License, Version 2.0",
    version=VERSION,
    packages=["datasette_sqlite_regex"],
    entry_points={"datasette": ["sqlite_regex = datasette_sqlite_regex"]},
    install_requires=["datasette", "sqlite-regex"],
    extras_require={"test": ["pytest"]},
    python_requires=">=3.7",
)