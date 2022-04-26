// Copyright 2020 Tyler Neely, Alex Regueiro
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// Ensure the libraries are linked in, despite it not being used directly
#[cfg(feature = "bzip2")]
extern crate bzip2_sys;
#[cfg(feature = "zlib")]
extern crate libz_sys;
#[cfg(feature = "zstd")]
extern crate zstd_sys;

#[cfg(feature = "bzip2")]
#[no_mangle]
pub fn bz_internal_error(errcode: libc::c_int) {
    panic!("bz internal error: {}", errcode);
}

use autocxx::prelude::*;

include_cpp! {
    #include "rocksdb/advanced_options.h"
    #include "rocksdb/cache.h"
    #include "rocksdb/cleanable.h"
    #include "rocksdb/compaction_filter.h"
    #include "rocksdb/compaction_job_stats.h"
    #include "rocksdb/comparator.h"
    #include "rocksdb/compression_type.h"
    #include "rocksdb/concurrent_task_limiter.h"
    #include "rocksdb/configurable.h"
    #include "rocksdb/convenience.h"
    #include "rocksdb/customizable.h"
    #include "rocksdb/data_structure.h"
    #include "rocksdb/db.h"
    #include "rocksdb/env_encryption.h"
    #include "rocksdb/env.h"
    #include "rocksdb/experimental.h"
    #include "rocksdb/file_checksum.h"
    #include "rocksdb/file_system.h"
    #include "rocksdb/filter_policy.h"
    #include "rocksdb/flush_block_policy.h"
    #include "rocksdb/functor_wrapper.h"
    #include "rocksdb/iostats_context.h"
    #include "rocksdb/io_status.h"
    #include "rocksdb/iterator.h"
    #include "rocksdb/ldb_tool.h"
    #include "rocksdb/listener.h"
    #include "rocksdb/memory_allocator.h"
    #include "rocksdb/memtablerep.h"
    #include "rocksdb/merge_operator.h"
    #include "rocksdb/metadata.h"
    #include "rocksdb/options.h"
    #include "rocksdb/perf_context.h"
    #include "rocksdb/perf_level.h"
    #include "rocksdb/persistent_cache.h"
    #include "rocksdb/rate_limiter.h"
    #include "rocksdb/rocksdb_namespace.h"
    #include "rocksdb/secondary_cache.h"
    #include "rocksdb/slice.h"
    #include "rocksdb/slice_transform.h"
    #include "rocksdb/snapshot.h"
    #include "rocksdb/sst_dump_tool.h"
    #include "rocksdb/sst_file_manager.h"
    #include "rocksdb/sst_file_reader.h"
    #include "rocksdb/sst_file_writer.h"
    #include "rocksdb/sst_partitioner.h"
    #include "rocksdb/statistics.h"
    #include "rocksdb/stats_history.h"
    #include "rocksdb/status.h"
    #include "rocksdb/system_clock.h"
    #include "rocksdb/table.h"
    #include "rocksdb/table_properties.h"
    #include "rocksdb/threadpool.h"
    #include "rocksdb/thread_status.h"
    #include "rocksdb/trace_reader_writer.h"
    #include "rocksdb/trace_record.h"
    #include "rocksdb/trace_record_result.h"
    #include "rocksdb/transaction_log.h"
    #include "rocksdb/types.h"
    #include "rocksdb/unique_id.h"
    #include "rocksdb/universal_compaction.h"
    #include "rocksdb/version.h"
    #include "rocksdb/wal_filter.h"
    #include "rocksdb/write_batch_base.h"
    #include "rocksdb/write_batch.h"
    #include "rocksdb/write_buffer_manager.h"

    safety!(unsafe)
    generate!("rocksdb::DB")
}
