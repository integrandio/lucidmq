from setuptools import setup

setup(
    name='LucidMQ-py',
    version='0.1.0',    
    description='LucidMQ Client code',
    url='https://github.com/lucidmq/lucidmq',
    license='MIT',
    packages=['lucidmq-py'],
    install_requires=['pycapnp'],
)