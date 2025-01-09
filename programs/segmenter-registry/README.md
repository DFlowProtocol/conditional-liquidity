# Segmenter Registry

The Segmenter Registry is an on-chain registry that tracks segmenter public keys.

Each registry account maintains a list of segmenter public keys that have been elected to the registry.

### Deployments
| Environment | Address | Version |
| ----------- | ------- | ------- |
| Solana Mainnet | [SRegZsVZDDqwc7W5iMUSsmKNnXzgfczKzFpimRp5iWw](https://explorer.solana.com/address/SRegZsVZDDqwc7W5iMUSsmKNnXzgfczKzFpimRp5iWw) | 1.0.0 |

### Usage

#### Initialize the program
The config account must be initialized before segmenters can be added. To initialize the registry, use
the `initialize` instruction. This instruction can only be processed once.

#### Create a registry
To create a registry, use the `create_registry` instruction, signed by any signer.

#### Add a segmenter
To add a segmenter, use the `add_segmenter` instruction with the public key of the segmenter as a parameter, signed by the admin. If the registry has reached it's max capacity of 64 keys or if the key already exists in it, the transaction will fail.

#### Remove a segmenter
To remove a segmenter, use the `remove_segmenter` instruction with the public key of the segmenter as a parameter, signed by the admin.

#### Change the admin
To change the admin, use the `change_admin` instruction with the public key of the new admin as a parameter, signed by the current admin.

### Developing
1. Install Anchor (https://www.anchor-lang.com/docs/installation). Use the version specified in `Anchor.toml`.
2. From the top level of the repository, run `yarn install`. This will install Node.js dependencies that are needed to run the integration tests.
3. From the top level of the repository, run `anchor test`. This will build the program and run the integration tests.
