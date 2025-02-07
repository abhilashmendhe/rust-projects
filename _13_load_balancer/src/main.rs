use std::{collections::{HashMap, HashSet}, hash::Hash};

// Input -> Request === Node -> Output

trait LoadBalancer {
    fn balance_load(&self, request: Request) -> Destination;
    fn register(&mut self, request_type: RequestType, service: Service);
}


struct LeastConnectionLoadBalancer {    // concreate class that implements tarit LoadBalancer
    // for particular type of request, there is a set of destinations which has subsribed to this request
    // for e.g. group sevice request node, the group service will say that I can take request and I have N number of nodes
    // that can handle these request.
    name: String,
    service_map: HashMap<RequestType, Service>,
}
impl LoadBalancer for LeastConnectionLoadBalancer {
    fn balance_load(&self, request: Request) -> Destination {
        let service = self.service_map.get(&request.request_type).unwrap();
        let destiantions = &service.destinations;
        let dest = destiantions.iter().min_by_key(|v|v.request_currently_serving);
        dest.unwrap().clone()
    }
    fn register(&mut self, request_type: RequestType, service: Service) {
        self.service_map.insert(request_type, service);
    }
}

struct RoundRobinLoadBalancer{
    name: String,
    service_map: HashMap<RequestType, Service>,
    
}

impl LoadBalancer for RoundRobinLoadBalancer {
    fn balance_load(&self, request: Request) -> Destination {
        let service = self.service_map.get(&request.request_type).unwrap();
        let destiantions = &service.destinations;
        let dest = destiantions.iter().min_by_key(|v|v.request_currently_serving);
        dest.unwrap().clone()
    }
    fn register(&mut self, request_type: RequestType, service: Service) {
        self.service_map.insert(request_type, service);
    }
}

struct Request {
    id: String, 
    request_type: RequestType,
    parameters: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum RequestType {

}

struct Service {
    destinations: HashSet<Destination>,
    // node_map: HashMap<Service, HashSet<Destination>>,
}

impl Service {
    fn add_destination(&mut self, destination: Destination) {
        self.destinations.insert(destination);
    }
    fn remove_destination(&mut self, destination: Destination) {
        self.destinations.remove(&destination);
    }
}

#[derive(Debug, Clone)]
struct Destination {
    ip_addr: String,
    weight: f32,
    request_currently_serving: i32,
    threshld: i32,
}

// Implement PartialEq manually
impl PartialEq for Destination {
    fn eq(&self, other: &Self) -> bool {
        self.ip_addr == other.ip_addr 
            // && self.weight.to_bits() == other.weight.to_bits() // Convert f32 to u32 for equality
            // && self.request_currently_serving == other.request_currently_serving
            // && self.threshld == other.threshld
    }
}

// implement Eq 
impl Eq for Destination {} // Safe because we ensured weight uses to_bits() in PartialEq

impl Hash for Destination {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ip_addr.hash(state);
        // self.weight.to_bits().hash(state); // converting f32 to u32 or hashing
        // self.request_currently_serving.hash(state);
        // self.threshld.hash(state);
    }
}

impl Destination {
    fn new() -> Destination {
        Destination {
            ip_addr: "".to_string(),
            weight: 0.0,
            request_currently_serving: 0,
            threshld: 0
        }
    }
    fn accept_request(&mut self, request: Request) -> bool {
        if self.threshld <= self.request_currently_serving {
            self.request_currently_serving+=1;
            true
        } else {
            false
        }
    }
    fn complete_request(&mut self) {
        self.request_currently_serving-=1;
    }

}


fn main() {

    println!("Hello, world!");
}
