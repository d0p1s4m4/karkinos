use pnet;

fn list_all_interfaces() -> Vec<datalink::NetworkInterface>
{
    datalink::interfaces();
}