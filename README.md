[English](README.md) | [日本語](README-ja.md)

<h1 align="center"> seeder </h1>

Seeder is a simple application that can be run from the console to populate tables with data.

A seeder application is used to insert data into tables. This application can register data defined in separate files into tables all at once. It can also insert a specified number of random data entries, which is useful for setting up test environments.

## Prerequisites

- This application is intended for use with PostgreSQL.
- Database connection information should be set in a .env file.
- The application is not distributed as a binary file, so you need to build it in your own environment.

# Environment Setup

## How to Build

In an environment with Rust installed, run the following command:

```shell
cargo build --release
```

Place the built executable file in the desired directory.

The built file will be generated at the following path:

```shell
./target/release/seeder
```

## Database Access Configuration

You need to configure database access information beforehand.

Follow these steps to set it up:

1. Place a .env file in the same directory as the built seeder executable.
2. Write the database access settings in the .env file in the following format:

```env
DATABASE_URL=postgres://username:password@hostname:port/db_name
```

# Preparation

## Prerequisites for Preparation

Tables must already be registered in the database.

For table registration, you may find [migrate](https://github.com/kip2/migrate) useful.

Include the registered table information in the database access settings in the .env file.

Here is the format for the .env file:

```env
DATABASE_URL=postgres://username:password@hostname:port/db_name
```

---

# How to Use

## Summary of Available Commands

The following commands are available. Detailed usage explanations are provided in separate sections.

```sh
# Insert data into tables from specified files
# file-path: Path to the file containing the data to be inserted
./seeder -f <file-path>
# Multiple files are also supported
./seeder -f <file-path1> <file-path2>

# Insert random data into tables
# file-path: Path to the file containing the column information of the target table
# n: Number of random data entries to generate
./seeder -r <file-path> <n>

# Generate a template for the JSON file used with both -f and -r options
# file-path: Path where the template file will be generated
./seeder -c <file-path>
```

---

## Inserting Data into Tables (-f Option)

Data insertion into tables is done using predefined data in JSON files.

There are two steps required:

- Preparing the file
- Executing the command

### Preparing the File

Define the data to be inserted in a JSON file.

A command to generate a template JSON file is also provided.

```sh
# Generate a template JSON file at the specified path
# file-path: Path where the template file will be generated
./seeder -c <file-path>
```

Fill in the necessary information in the generated template file.

Keep in mind the following:

- The available `data_type` values are:
  - `int`
  - `float`
  - `string`
  - `date` 

Here is an example of a JSON file definition:

```json
{
    "table_name": "computer_parts",
    "table_columns": [
        {
            "data_type": "string",
            "column_name": "name"
        },
        {
            "data_type": "int",
            "column_name": "lifespan"
        }
    ],
    "table_rows": [
        [
            "Ryzen 9 5900X",
            5
        ]
    ]
}
```

### Executing the Command

Once you have prepared the definition file, execute the command.

You can also execute multiple files.

```sh
# file-path: Path to the file containing the data to be inserted
./seeder -f <file-path>

# Multiple files are also supported
./seeder -f <file-path1> <file-path2>
```

---

## Inserting Random Data (-r Option)

It is possible to insert multiple random data entries into tables, useful for testing purposes.

There are two steps required:

- Preparing the file
- Executing the command

### Preparing the File

Define the column information of the target table in a JSON file.

A command to generate a template JSON file is also provided.

```sh
# Generate a template JSON file at the specified path
# file-path: Path where the template file will be generated
./seeder -c <file-path>
```

Fill in the necessary column information in the generated template file.

Keep in mind the following:

- The available `data_type` values are:
  - `int`
  - `float`
  - `string`
  - `date` 
- Leave the "table_rows" field empty, as it will be used during execution.

Here is an example of a JSON file definition:

```json
{
    "table_name": "computer_parts",
    "table_columns": [
        {
            "data_type": "string",
            "column_name": "name"
        },
        {
            "data_type": "int",
            "column_name": "lifespan"
        }
    ],
    "table_rows": []
}
```

### Executing the Command

Once you have prepared the JSON file, execute the command.

Include the path to the file defining the column data and the number of random data entries to generate.

```sh
# file-path: Path to the file containing the column information of the target table
# n: Number of random data entries to generate
./seeder -r <file-path> <n>
```

---

## Creating Template Files (-c Option)

This command generates template files required for running the seeder.

Run the following command, and then fill in the necessary information for either the -f or -r option.

```sh
# Generate a template JSON file at the specified path
# file-path: Path where the template file will be generated
./seeder -c <file-path>
```

---

# Help

If you need help with the commands, refer to the help options.

You can access the help with the following commands:

```sh
./seeder -h

# Or
./seeder --help
```
