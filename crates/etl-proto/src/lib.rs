pub mod common {
    tonic::include_proto!("etl.common");
}

pub mod source {
    tonic::include_proto!("etl.source");
}

pub mod transform {
    tonic::include_proto!("etl.transform");
}

pub mod lookup {
    tonic::include_proto!("etl.lookup");
}

pub mod sink {
    tonic::include_proto!("etl.sink");

    impl WriteOptions {
        pub fn at_least_once() -> Self {
            Self {
                require_ack: true,
                timeout_ms: 30000,
                guarantee: DeliveryGuarantee::AtLeastOnce as i32,
            }
        }

        pub fn exactly_once() -> Self {
            Self {
                require_ack: true,
                timeout_ms: 60000,
                guarantee: DeliveryGuarantee::ExactlyOnce as i32,
            }
        }
    }
}

pub mod registry {
    tonic::include_proto!("etl.registry");
}

pub mod checkpoint {
    tonic::include_proto!("etl.checkpoint");
}

pub mod router {
    tonic::include_proto!("etl.router");
}

pub mod raft {
    tonic::include_proto!("raft");
}

pub mod backup {
    tonic::include_proto!("etl.backup");
}
