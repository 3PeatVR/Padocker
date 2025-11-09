**English version below**

# 🐳 PADOCKER 🐳

C'est comme Docker mais c'est Padocker ! 🐋

## Sommaire

- [Introduction](#introduction)
- [Pré-requis](#pre-requis)
- [Commandes](#commandes)

## Introduction

Ce projet est un clône de Docker entièrement écrit en Rust. Voici l'arborescence : 
```
.
├── Cargo.lock
├── Cargo.toml
├── containers <- Dossier où sont stockés les fichiers des containers.
├── README.md
└── src
    ├── cgroups.rs <- Gère les cgroups
    ├── cli.rs <- gère la partie CLI
    ├── container.rs <- Gère la création et suppression de containers
    ├── ls.rs <- Renvoie une liste détaillée des containers sauvegardés
    └── main.rs <- fonction principale
```

## Pré-requis

Pour profiter pleinement des containers et pour les sauvegarder, il faut installer `debootstrap` à l'aide de votre gestionnaire de paquet préféré.

## Commandes

Padocker permet donc de créer des environnements d'exécution légers à l'aide de namespaces (comme les containers Docker 🐳). Il est possible de sauvegarder si l'on souhaite les fichiers de son container, qui seront stockés dans le dossier `containers`. Voici la liste des commandes disponibles :

```
sudo cargo run -- run

(--name <name> si le nom est absent le container aura le nom de la commande s'il est sauvegardé)

(--fs si on veut sauvegarder les fichiers)

(--memory_limit <int> pour désigner la limite de mémoire en Mio si on activé les filesystem)

<program>
```
afin de lancer dans un container la commande `program`.

- `sudo cargo run -- delete (--name <name> si on veut supprimer un container à partir de son nom) (--all si on veut supprimer tous les containers)`

- `sudo cargo run -- ls` pour lister les containers sauvegardés dans `containers`

- `sudo cargo run -- hello` Honnêtement c'était un test pour voir si le CLI marchait et j'ai oublié de l'enlever.

# English Version

# 🐳 PADOCKER 🐳

It is like Docker but it is Padocker (this pun was better in French ...) ! 🐋

## Summary

- [Intro](#introd)
- [Prerequisites](#prerequisites)
- [Commands](#commands)

## Intro

This project is a clone of Docker written entirely in Rust. Here's its tree structure:
```
.
├── Cargo.lock
├── Cargo.toml
├── containers <- Folder where container files are stored
├── README.md
└── src
    ├── cgroups.rs <- Managing cgroups
    ├── cli.rs <- Managing CLI
    ├── container.rs <- Managing creation & suprression of containers
    ├── ls.rs <- Displays a detailed list of saved containers
    └── main.rs <- main function
```

## Prerequisites

To take full advantage of containers and to save them, you have to install `debootstrap` with your preferred package manager.

## Commands

Padocker allows you to create light execution environments with the help of namespaces (just like Docker containers 🐳). It is possible to save the files of your container if you wish, they will be saved in the `containers` folder. Here is the list of commands:

```
sudo cargo run -- run

(--name <name> if the name is missing the container will have the name of the command if the container is saved)

(--fs if you want to save the files of the container)

(--memory_limit <int> to put a limit memory in Mio if filesystem is activated)

<program>
```
in order to run in the container the command `program`.

- `sudo cargo run -- delete (--name <name> if we want to delete a container by its name) (--all if we want to delete all containers)`

- `sudo cargo run -- ls` for listing all containers in `containers`

- `sudo cargo run -- hello` Honestly it was just a test to see if the CLI was working and I forgot to remove it 😶‍🌫️.
