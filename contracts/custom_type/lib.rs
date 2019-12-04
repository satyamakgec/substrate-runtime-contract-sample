#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use scale::KeyedVec as _;
use scale::{ Encode };
use ink_prelude::{
    format,
    vec::Vec,
};

use ink_lang2 as ink;

/// Define hashing functions required for hashing the key to read a Value from runtime storage
mod hashing {
    /// Do a XX 128-bit hash and place result in `dest`.
    pub fn twox_128_into(data: &[u8], dest: &mut [u8; 16]) {
        use ::core::hash::Hasher;
        let mut h0 = twox_hash::XxHash::with_seed(0);
        let mut h1 = twox_hash::XxHash::with_seed(1);
        h0.write(data);
        h1.write(data);
        let r0 = h0.finish();
        let r1 = h1.finish();
        use byteorder::{ByteOrder, LittleEndian};
        LittleEndian::write_u64(&mut dest[0..8], r0);
        LittleEndian::write_u64(&mut dest[8..16], r1);
    }

    /// Do a XX 128-bit hash and return result.
    pub fn twox_128(data: &[u8]) -> [u8; 16] {
        let mut r: [u8; 16] = [0; 16];
        twox_128_into(data, &mut r);
        r
    }

    /// Do a Blake2 256-bit hash and place result in `dest`.
    pub fn blake2_256_into(data: &[u8], dest: &mut [u8; 32]) {
        dest.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], data).as_bytes());
    }

    /// Do a Blake2 256-bit hash and return result.
    pub fn blake2_256(data: &[u8]) -> [u8; 32] {
        let mut r = [0; 32];
        blake2_256_into(data, &mut r);
        r
    }
}

/// Contract to demonstrate reading a custom struct directly from runtime storage
#[ink::contract(version = "0.1.0")]
mod custom_type {
    #[ink(storage)]
    struct CustomRuntimeStorageTypeContract {}

    /// Copy of the custom type defined in `/runtime/src/template.rs`.
    ///
    /// # Requirements
    /// In order to decode a value of that type from the runtime storage:
    ///   - The type must match exactly the custom type defined in the runtime
    ///   - It must implement `Decode`, usually by deriving it as below
    ///   - It should implement `Metadata` for use with `generate-metadata` (required for the UI).
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
    pub struct Foo {
        id: u32,
        data: Vec<u8>,
    }

    impl CustomRuntimeStorageTypeContract {
        /// Constructor initializes the contract
        #[ink(constructor)]
        fn new(&mut self) {
        }

        /// Attempts to read an instance of the custom struct from runtime storage
        ///
        /// Returns `None` if the key does not exist, or it failed to decode the value.
        #[ink(message)]
        fn read_custom_runtime(&self) -> Option<Foo> {
            let mut key = [0u8; 32];
            // A storage key is constructed as `Twox128(module_prefix) ++ Twox128(storage_prefix)`
            let module_prefix = hashing::twox_128(&b"TemplateModule"[..]);
            let storage_prefix = hashing::twox_128(&b"FooStore"[..]);
            key[0..16].copy_from_slice(&module_prefix);
            key[16..32].copy_from_slice(&storage_prefix);
            self.env().println(&format!("Storage key: {:?}", key));

            // Attempt to read and decode the value directly from the runtime storage
            let result = self.env().get_runtime_storage::<Foo>(&key[..]);
            match result {
                Ok(foo) => {
                    // Return the successfully decoded instance of `Foo`
                    Some(foo)
                },
                Err(err) => {
                    // Either the key did not exist or it failed to decode.
                    // Print the reason for the error and return None.
                    // *Note:* `println` should only be used for debugging, not in production contracts.
                    self.env().println(&format!("Error reading runtime storage: {:?}", err));
                    None
                }
            }
        }

        #[ink(message)]
        fn read_custom_runtime_map(&self, index: u8, address: AccountId) -> Option<Foo> {
            // Twox128(module_prefix) ++ Twox128(storage_prefix) ++ Hasher(encode(key))
            let module_prefix = hashing::twox_128(&b"TemplateModule"[..]);
            let storage_prefix = hashing::twox_128(&b"LooStore"[..]);
            let hashed_key = hashing::blake2_256(&(index,address).encode());

            let mut storage_key = Vec::with_capacity(
			    module_prefix.len() + storage_prefix.len() + hashed_key.as_ref().len()
		    );

            storage_key.extend_from_slice(&module_prefix[..]);
            storage_key.extend_from_slice(&storage_prefix[..]);
            storage_key.extend_from_slice(hashed_key.as_ref());

            self.env().println(&format!("Storage key: {:?}", storage_key));

            // Attempt to read and decode the value directly from the runtime storage
            let result = self.env().get_runtime_storage::<Foo>(&storage_key[..]);
            match result {
                Ok(foo) => {
                    // Return the successfully decoded instance of `Foo`
                    Some(foo)
                },
                Err(err) => {
                    // Either the key did not exist or it failed to decode.
                    // Print the reason for the error and return None.
                    // *Note:* `println` should only be used for debugging, not in production contracts.
                    self.env().println(&format!("Error reading runtime storage: {:?}", err));
                    None
                }
            }
        }

        #[ink(message)]
        fn read_custom_runtime_linked_map(&self, address: AccountId) -> Option<Foo> {
            // Twox128(module_prefix) ++ Twox128(storage_prefix) ++ Hasher(encode(key))
            let module_prefix = hashing::twox_128(&b"TemplateModule"[..]);
            let storage_prefix = hashing::twox_128(&b"PooStore"[..]);
            let hashed_key = hashing::blake2_256(&address.encode());

            let mut storage_key = Vec::with_capacity(
			    module_prefix.len() + storage_prefix.len() + hashed_key.as_ref().len()
		    );

            storage_key.extend_from_slice(&module_prefix[..]);
            storage_key.extend_from_slice(&storage_prefix[..]);
            storage_key.extend_from_slice(hashed_key.as_ref());

            self.env().println(&format!("Storage key: {:?}", storage_key));

            // Attempt to read and decode the value directly from the runtime storage
            let result = self.env().get_runtime_storage::<Foo>(&storage_key[..]);
            match result {
                Ok(foo) => {
                    // Return the successfully decoded instance of `Foo`
                    Some(foo)
                },
                Err(err) => {
                    // Either the key did not exist or it failed to decode.
                    // Print the reason for the error and return None.
                    // *Note:* `println` should only be used for debugging, not in production contracts.
                    self.env().println(&format!("Error reading runtime storage: {:?}", err));
                    None
                }
            }
        }
    }
}
