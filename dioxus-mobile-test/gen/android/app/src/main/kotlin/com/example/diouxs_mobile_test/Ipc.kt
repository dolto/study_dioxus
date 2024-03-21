/* THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!! */

// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

@file:Suppress("unused")

package com.example.diouxs_mobile_test

import android.webkit.*

class Ipc {
    @JavascriptInterface
    fun postMessage(message: String) {
        this.ipc(message)
    }

    companion object {
        init {
            System.loadLibrary("diouxs_mobile_test")
        }
    }

    private external fun ipc(message: String)

    
}
