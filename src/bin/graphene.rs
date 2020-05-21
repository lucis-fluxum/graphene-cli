//! Main entry point for GrapheneCli

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use graphene_cli::application::APPLICATION;

/// Boot GrapheneCli
fn main() {
    abscissa_core::boot(&APPLICATION);
}
