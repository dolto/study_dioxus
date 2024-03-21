# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class com.example.diouxs_mobile_test.* {
  native <methods>;
}

-keep class com.example.diouxs_mobile_test.WryActivity {
  public <init>(...);

  void setWebView(com.example.diouxs_mobile_test.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class com.example.diouxs_mobile_test.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class com.example.diouxs_mobile_test.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void setAutoPlay(...);
  void setUserAgent(...);
  void evalScript(...);
}

-keep class com.example.diouxs_mobile_test.RustWebChromeClient,com.example.diouxs_mobile_test.RustWebViewClient {
  public <init>(...);
}