/* Virt Blocks
 *
 * Copyright (C) 2019 Red Hat, Inc.
 *
 * This software is distributed under the terms of the MIT License.
 * See the LICENSE file in the top level directory for details.
 */

#pragma once

typedef struct VirtBlocksUltra VirtBlocksUltra;

typedef void (*VirtBlocksUltraCb)(int, void*);
typedef void (*VirtBlocksFreeCb)(void*);


VirtBlocksUltra *virtblocks_ultra_new(void);
void virtblocks_ultra_free(VirtBlocksUltra* ultra);

void virtblocks_ultra_set_cb(VirtBlocksUltra* ultra,
                             VirtBlocksUltraCb callback,
                             void *opaque,
                             VirtBlocksFreeCb free_cb);
void virtblocks_ultra_unset_cb(VirtBlocksUltra* ultra);

void virtblocks_ultra_call_me(VirtBlocksUltra* ultra);
