//! # Renderer
//!
//! Rendering Engine for wtvr3d. Uses WebGL through the `web-sys` crate.  

pub mod material;

pub mod uniform;

pub mod buffer;

pub use buffer::Buffer;

pub mod shader_data_type;

use crate::component::camera::Camera;
use crate::component::mesh::Mesh;
use nalgebra::Matrix4;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use uniform::Uniform;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

/// ## Renderer
/// 
/// Renderer for `wtvr3D`. Renders meshes from the point of view of a `Camera`  
/// Every Mesh must be registered with the `Renderer` before being rendered.
/// Otherwise, the mesh won't be included in the render.
/// 
/// A Renderer needs a `WebGlRenderingContext` to render to, and a reference to the
/// associated `HtmlCanvasElement`.
pub struct Renderer<'a> {

    /// Mesh repository where `Mesh`es are registered.
    mesh_repository: HashMap<u32, Vec<Rc<RefCell<Mesh<'a>>>>>,

    /// The current WebGlRenderingContext to render to.
    webgl_context: WebGlRenderingContext,

    /// The target `HtmlCanvasElement` 
    canvas: HtmlCanvasElement,

    /// Internal counter to attribute unique Ids to different `Material`s
    next_material_id: u32,

    /// Camera reference used for rendering.
    main_camera: Rc<RefCell<Camera>>,
}

impl<'a> Renderer<'a> {

    /// Constructor. Must be provided a Canvas reference, a `WebGlRenderingContext` and a
    /// valid Camera to be used to render the scene.
    pub fn new(
        camera: Camera,
        canvas: HtmlCanvasElement,
        context: WebGlRenderingContext,
    ) -> Renderer<'a> {
        Renderer {
            mesh_repository: HashMap::new(),
            webgl_context: context,
            canvas: canvas,
            next_material_id: 0,
            main_camera: Rc::new(RefCell::new(camera)),
        }
    }

    /// Registers a new `Mesh` in the mesh repository. Also provides an Id for its `Material`
    /// if it doesn't already have one.  
    /// It also looks up for any `Uniform` or `Attribute` location for the associated `Material`
    /// Therefore, this should be done only once at initialization time.
    pub fn register_mesh(&mut self, mesh: &Rc<RefCell<Mesh<'a>>>) -> () {
        let mut mesh_mut = mesh.borrow_mut();
        mesh_mut.lookup_locations(&self.webgl_context);
        let id = mesh_mut.material.get_parent_id(self.next_material_id);
        if self.mesh_repository.contains_key(&id) {
            let vec = self.mesh_repository.get_mut(&id).unwrap();
            vec.push(Rc::clone(mesh));
        } else {
            self.mesh_repository.insert(id, vec![Rc::clone(mesh)]);
        }
    }

    /// Resizes the canvas internal size to match the display resolution and ratio.  
    /// Also updates the WebGl Viewport to match.
    /// 
    /// ⚠️ might be removed in favor of all-JS version.
    pub fn resize_canvas(&mut self) -> () {
        let display_width = self.canvas.client_width() as u32;
        let display_height = self.canvas.client_height() as u32;
        if self.canvas.width() != display_width || self.canvas.height() != display_height {
            self.canvas.set_width(display_width);
            self.canvas.set_height(display_height);
        }
        self.webgl_context
            .viewport(0, 0, display_width as i32, display_height as i32);
        self.main_camera
            .borrow_mut()
            .set_aspect_ratio(display_width as f32 / display_height as f32)
    }

    /// Renders all the objects registered in the Mesh Repository and prints them to the Canvas.component
    /// 
    /// The opaque objects will be rendered before the transparent ones (ordered by depth), and every object will be sorted
    /// by `Material` id to optimize performance.
    pub fn render_objects(&self) {
        let vp_matrix = self.main_camera.borrow_mut().compute_vp_matrix().clone();
        self.webgl_context.clear_color(0., 0., 0., 0.);
        self.webgl_context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        self.webgl_context.enable(WebGlRenderingContext::CULL_FACE);
        self.webgl_context.enable(WebGlRenderingContext::DEPTH_TEST);

        let meshes = self.sort_objects();
        let mut current_id = u32::max_value();
        for mesh_rc in meshes {
            let mut mesh = mesh_rc.borrow_mut();
            let material_id = mesh.material.get_parent_id(0);
            if material_id != current_id {
                current_id = material_id;
                self.webgl_context
                    .use_program(Some(mesh.material.get_parent().borrow().get_program()));
                self.set_camera_uniform(&mut mesh, vp_matrix.clone()).ok();
            }
            self.draw_mesh(&mesh);
        }
    }

    /// Draws a single mesh to the Canvas.  
    /// Meant to be used by `Self.render_objects`
    fn draw_mesh(&self, mesh: &Mesh<'a>) {
        for buffer in mesh.get_buffers() {
            buffer.enable_and_bind_attribute(&self.webgl_context);
        }
        self.webgl_context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            mesh.get_vertex_count(),
        );
    }

    /// Sets the global camera uniform for the whole scene  
    /// Meant to be used by `Self.render_objects`
    fn set_camera_uniform(&self, mesh: &mut Mesh, vp_matrix: Matrix4<f32>) -> Result<(), String> {
        let camera_uniform_location = mesh
            .material
            .get_parent()
            .borrow()
            .global_uniform_locations
            .vp_matrix_location
            .clone();
        let vp_matrix_uniform = Uniform::new_with_location(
            uniform::VP_MATRIX_NAME,
            camera_uniform_location,
            Box::new(vp_matrix),
        );
        vp_matrix_uniform.set_to_context(&self.webgl_context)
    }

    /// Sorts objects by transparency and by depth for transparent objects.
    fn sort_objects(&self) -> Vec<Rc<RefCell<Mesh<'a>>>> {
        let mut opaque_meshes = Vec::new();
        let mut transparent_meshes = Vec::new();
        for (_, mesh_vec) in &self.mesh_repository {
            for mesh in mesh_vec {
                if mesh.borrow().material.is_transparent() {
                    transparent_meshes.push(Rc::clone(&mesh));
                } else {
                    opaque_meshes.push(Rc::clone(&mesh));
                }
            }
        }
        // Sort transparent objects depending on depth
        opaque_meshes.append(&mut transparent_meshes);
        opaque_meshes
    }
}
