/*
 * Copyright 2021 The Raksha Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub mod antlr_gen {

    // This macro exists as a way to make use of 
    // aesthetically pleasing concatenation without making
    // temporary variables or giving up code reuse.
    macro_rules! prefix { () => {"../../../target/antlr_gen/"} }

    pub mod authlogiclexer {
        include!(concat!(prefix!(), "authlogiclexer.rs"));
    }

    pub mod authlogiclistener {
        include!(concat!(prefix!(), "authlogiclistener.rs"));
    }

    pub mod authlogicparser {
        include!(concat!(prefix!(), "authlogicparser.rs"));
    }
}
