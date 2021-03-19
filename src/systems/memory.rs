use super::System;
use crate::config::CoreConfig;
use bumpalo::Bump as BumpAllocator;
use indicatif::HumanBytes;
use lifeguard::{pool, Pool, StartingSize};
use log::info;

pub struct MemorySystem {
    pub string_pool: Pool<String>,
    pub bump_allocator: BumpAllocator,
}

impl System for MemorySystem {
    type Args = ();

    fn initialize(cfg: &mut CoreConfig, _: &Self::Args) -> Self {
        info!(
            "Creating string pool with {} pre allocated entries...",
            cfg.memory_config.default_string_pool_size
        );
        let string_pool = pool()
            .with(StartingSize(cfg.memory_config.default_string_pool_size))
            .build();

        info!(
            "Creating bump allocator with {} capacity...",
            HumanBytes(cfg.memory_config.default_memory_pool_size as _)
        );
        let bump_allocator =
            BumpAllocator::with_capacity(cfg.memory_config.default_memory_pool_size);

        // todo: config
        Self {
            string_pool,
            bump_allocator,
        }
    }
}
