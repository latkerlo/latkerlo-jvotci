/*
Copyright (c) 2023-2024 latkerlo (https://github.com/latkerlo)
Licensed under the MIT License
*/
class DecompositionError extends Error {
    constructor(message) {
        super(message);
        this.name = "DecopositionError";
    }
}
class InvalidClusterError extends Error {
    constructor(message) {
        super(message);
        this.name = "InvalidClusterError";
    }
}
class NoLujvoFoundError extends Error {
    constructor(message) {
        super(message);
        this.name = "NoLujvoFoundError";
    }
}
class NonLojbanCharacterError extends Error {
    constructor(message) {
        super(message);
        this.name = "NonLojbanCharacterError";
    }
}
class NotBrivlaError extends Error {
    constructor(message) {
        super(message);
        this.name = "NotBrivlaError";
    }
}
class NotZihevlaError extends Error {
    constructor(message) {
        super(message);
        this.name = "NotZihevlaError";
    }
}
