import setuptools

with open("Readme.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="rusteomics_proteomics_io",
    version="0.1.0",
    author="Dirk Winkelhardt",
    author_email="dirk.winkelhardt@rub.de",
    description="I/O module for reading and writing common proteomics file formats.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/rusteomics/proteomics-io-py",
    packages=setuptools.find_packages(),
    include_package_data=True,
    test_suite="tests",
    classifiers=[
    ],
    python_requires='>=3.7',
    install_requires = [
    ]
)