// this is a driver module or implementation layer!
// this layer interact with database and in return giving additional layer of services to the gateway
// gateway the pass throught the port(abstraction layer) and providing interface for our usecases to interact
// with entities;
use mongodb::Collection;
pub mod task_drv;
pub mod tdlist_drv;
