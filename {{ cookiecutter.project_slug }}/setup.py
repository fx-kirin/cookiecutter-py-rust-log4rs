import sys
from setuptools import setup

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust>=0.9.2'])
    if errno:
        print("Please install the 'setuptools-rust>=0.9.2' package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding


setup(name="{{ cookiecutter.package_name }}",
      version="{{ cookiecutter.version }}",
      author="{{ cookiecutter.full_name }}",
      maintainer='{{ cookiecutter.full_name }}',
      author_email='{{ cookiecutter.email }}',
      maintainer_email='{{ cookiecutter.email }}',
      keywords='{{ cookiecutter.project_keywords }}',
      description='{{ cookiecutter.project_short_description }}',
      long_description='',
      packages=['{{ cookiecutter.project_slug }}'],
      rust_extensions=[
          RustExtension('{{ cookiecutter.project_slug }}.rust_binding', 'Cargo.toml', binding=Binding.PyO3, debug=False)
      ],
      license='BSD',
      url='https://github.com/{{ cookiecutter.github_username }}/{{ cookiecutter.package_name }}',
      zip_safe=False,
      setup_requires=['setuptools-rust>=0.9.2', 'pytest-runner'],
      install_requires=['setuptools-rust>=0.9.2'],
      tests_require=['pytest'],
      test_suite='tests',
      include_package_data=True,
      classifiers=[
          'Development Status :: 3 - Alpha',
          'Intended Audience :: Developers',
          'Intended Audience :: Financial and Insurance Industry',
          'Intended Audience :: Information Technology',
          'Intended Audience :: Science/Research',
          'Programming Language :: Python',
          'Programming Language :: Python :: 3.5',
          'Programming Language :: Python :: 3.6',
          'Programming Language :: Python :: 3 :: Only',
          'Programming Language :: Rust',
          'Operating System :: Microsoft :: Windows',
          'Operating System :: POSIX',
          'Operating System :: Unix',
          'Operating System :: MacOS :: MacOS X',
      ],
      )
