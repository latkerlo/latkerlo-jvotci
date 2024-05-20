"""
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
"""


class DecompositionError(Exception):
    def __init__(self, message=""):
        super().__init__(message)


class InvalidClusterError(Exception):
    def __init__(self, message=""):
        super().__init__(message)


class NoLujvoFoundError(Exception):
    def __init__(self, message=""):
        super().__init__(message)


class NonLojbanCharacterError(Exception):
    def __init__(self, message=""):
        super().__init__(message)


class NotBrivlaError(Exception):
    def __init__(self, message=""):
        super().__init__(message)


class NotZihevlaError(Exception):
    def __init__(self, message=""):
        super().__init__(message)
