use super::{CoreConfig, System};
use bumpalo::Bump as BumpAllocator;
use indicatif::HumanBytes;
use lifeguard::{pool, Pool, StartingSize};
use log::info;

pub struct MemorySystem {
    pub string_pool: Pool<String>,
    pub bump_allocator: BumpAllocator,
}

impl System for MemorySystem {
    fn initialize(cfg: &CoreConfig) -> Self {
        info!(
            "Creating string pool with {} preallocated entries...",
            cfg.mem_config.default_string_pool_size
        );
        let string_pool = pool()
            .with(StartingSize(cfg.mem_config.default_string_pool_size))
            .build();

        info!(
            "Creating bump allocator with {} capacity...",
            HumanBytes(cfg.mem_config.default_pool_allocator_size as _)
        );
        let bump_allocator =
            BumpAllocator::with_capacity(cfg.mem_config.default_pool_allocator_size);

        // todo: config
        Self {
            string_pool,
            bump_allocator,
        }
    }
}
