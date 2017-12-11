// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BootstrapAddDefault;

impl_skip_serialize!(BootstrapAddDefault);

impl ApiRequest for BootstrapAddDefault {
    const path: &'static str = "/bootstrap/add/default";
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BootstrapList;

impl_skip_serialize!(BootstrapList);

impl ApiRequest for BootstrapList {
    const path: &'static str = "/bootstrap/list";
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BootstrapRmAll;

impl_skip_serialize!(BootstrapRmAll);

impl ApiRequest for BootstrapRmAll {
    const path: &'static str = "/bootstrap/rm/all";
}
