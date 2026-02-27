use std::sync::Arc;

use crate::api_client::ApiClient;
use crate::config::Config;
use crate::http::Transport;
use crate::services::{
    AbuseService, AftermarketService, AgreementsService, AnsService, AuctionsService,
    CertificatesService, CountriesService, DomainsService, OrdersService, ParkingService,
    ShoppersService, SubscriptionsService,
};

#[derive(Clone)]
pub struct Client {
    api_client: Arc<ApiClient>,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            api_client: Arc::new(ApiClient::new(config, None)),
        }
    }

    pub fn with_transport(config: Config, transport: Arc<dyn Transport>) -> Self {
        Self {
            api_client: Arc::new(ApiClient::new(config, Some(transport))),
        }
    }

    pub fn api_client(&self) -> Arc<ApiClient> {
        self.api_client.clone()
    }

    pub fn abuse(&self) -> AbuseService {
        AbuseService::new(self.api_client.clone())
    }

    pub fn aftermarket(&self) -> AftermarketService {
        AftermarketService::new(self.api_client.clone())
    }

    pub fn agreements(&self) -> AgreementsService {
        AgreementsService::new(self.api_client.clone())
    }

    pub fn ans(&self) -> AnsService {
        AnsService::new(self.api_client.clone())
    }

    pub fn auctions(&self) -> AuctionsService {
        AuctionsService::new(self.api_client.clone())
    }

    pub fn certificates(&self) -> CertificatesService {
        CertificatesService::new(self.api_client.clone())
    }

    pub fn countries(&self) -> CountriesService {
        CountriesService::new(self.api_client.clone())
    }

    pub fn domains(&self) -> DomainsService {
        DomainsService::new(self.api_client.clone())
    }

    pub fn orders(&self) -> OrdersService {
        OrdersService::new(self.api_client.clone())
    }

    pub fn parking(&self) -> ParkingService {
        ParkingService::new(self.api_client.clone())
    }

    pub fn shoppers(&self) -> ShoppersService {
        ShoppersService::new(self.api_client.clone())
    }

    pub fn subscriptions(&self) -> SubscriptionsService {
        SubscriptionsService::new(self.api_client.clone())
    }
}
