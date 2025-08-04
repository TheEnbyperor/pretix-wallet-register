import React from "react";
import ReactDOM from "react-dom/client";
import App from "./src/App";
import 'bootstrap/dist/css/bootstrap.min.css';

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(<App />);

// let walletClient = wasm.WalletClient.new();
//
// walletClient.load_pk("5101", "1", `-----BEGIN PUBLIC KEY-----
// MIIBvzCCATMGByqGSM44BAEwggEmAoGBANudcKJHbCxGh7Bm9j55Cpp6OcfUpUJQ
// YZTcnSb0Zrc72Wq/tgXoJ/OFmsgyxgwWtjhg8J94rrLersfGUJejjahebXhB6kv0
// gfyJw1DyD9vtnpJejI9vxg6+BL1vc82yItCNAEFzqXZylZK58NceES+CHng3Dso6
// zZLUSTV+Ieq/Ah0AjaMThCPlD00bEhNXp1Maafe0dYhPgJ+DqigAxQKBgBnb6+GK
// xgzil/ABMhRy0D19sFu+AlD3+ybLp1zLecpvwljaE0XDNpPRAK0fXu87oqzvD2dI
// qgijIwjE9wwr8lv8XL8E4w67nGO2p1pLpV2fUQHDjBadomsAURpRB1GHEMq1LgGm
// x+N3Ov27BCHEiII/JvmxvmHt3D7tBt20t7U/A4GFAAKBgQCZfpkvmoiGw30VK6jz
// 1x01BOXj3TP4RMynMvhemQ9FRO35KnrJ9CQiNGFK4av9Qw6Nw6sQkgCwFPqhv49D
// FKOUQzCrZagvkQhbIhjWWxF3CGxZ0AxGY+XsdZbPjfVrIemwm8m4BaZnPMTyq4Q8
// HbttD0ltz0MC2+YIDXxHSVvAjQ==
// -----END PUBLIC KEY-----`);
//
// console.log(walletClient.debug());
