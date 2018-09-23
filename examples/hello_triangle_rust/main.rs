extern crate wgpu_native;
use wgpu_native::*;

fn main() {
    let instance = wgpu_create_instance();
    let adapter = wgpu_instance_get_adapter(
        instance,
        AdapterDescriptor {
            power_preference: PowerPreference::LowPower,
        },
    );
    let device = wgpu_adapter_create_device(
        adapter,
        DeviceDescriptor {
            extensions: Extensions {
                anisotropic_filtering: false,
            },
        },
    );
    let _vs = wgpu_device_create_shader_module(
        device,
        ShaderModuleDescriptor {
            code: include_bytes!("./../data/hello_triangle.vert.spv"),
        },
    );
    let _fs = wgpu_device_create_shader_module(
        device,
        ShaderModuleDescriptor {
            code: include_bytes!("./../data/hello_triangle.frag.spv"),
        },
    );
}
