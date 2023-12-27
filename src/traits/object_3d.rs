use bevy::prelude::*;

pub enum MeshLoadType<'a> {
	File { asset_server: &'a AssetServer },
	Code { meshes: &'a Assets<Mesh> }
}

pub trait Object3D: HasMesh {
	fn mesh_path(&self) -> Option<String>;
	fn mesh(&self) -> Option<Mesh>;
	fn mesh_load_type(&self) -> MeshLoadType;
	fn material(&self) -> StandardMaterial;
	fn translation(&self) -> Vec3;
	fn scale(&self) -> Vec3;

	fn mesh_handle(&self) -> Handle<Mesh> {
		use MeshLoadType::*;
		match self.mesh_load_type() {
			File { asset_server } => match self.mesh_path() {
				Some(mesh_path) => asset_server.load(mesh_path),
				None => panic!("no mesh path for Object3D w/ File load type")
			},
			Code { meshes } => match self.mesh() {
				Some(mesh) => meshes.add(mesh),
				None => panic!("no mesh for Object3D w/ Code load type")
			}
		}
	}

	fn material_handle(
		&self,
		materials: &mut Assets<StandardMaterial>
	) -> Handle<StandardMaterial> {
		materials.add(self.material())
	}

	fn transform(&self) -> Transform {
		Transform::from_translation(self.translation()).with_scale(self.scale())
	}

	fn pbr_bundle(
		&self,
		meshes: &Res<Assets<Mesh>>,
		materials: &mut ResMut<Assets<StandardMaterial>>
	) -> PbrBundle {
		PbrBundle {
			mesh: self.mesh_handle(),
			material: self.material_handle(materials.as_mut()),
			transform: self.transform(),
			..default()
		}
	}
}

pub trait HasMesh {}
