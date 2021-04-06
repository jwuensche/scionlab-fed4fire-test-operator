# scion-test-operator

This repository contains a small operator initiating an experiment run on nodes in part of our scionlab experiment topology.

## Why and What

This tool is thought to be used to initialize and run network experiments with many iterations. In order to initiate the desired state and run the experiments on the machines we use `ssh`, so the keys and users on the machines have to be configured accordingly.

## Configuration

Documentation is rather sparse at the moment please have a look at the `example.yml` and `scionlab.yml` file for an overview on how to define experiments and setups.
Many fields are static (the given hosts) as they are intended only to be used in this experiment run.
