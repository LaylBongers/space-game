use {
    std::path::{PathBuf},

    ggez::{
        Context,
        conf::{NumSamples},
        graphics::{self, Canvas, Font},
    },
    metrohash::{MetroHashMap},

    rivr::{
        Error, PanelId, ResourceError,
        attributes::{Vector2},
    },

    egtr,
};

struct FontCache {
    path: PathBuf,
    sizes: MetroHashMap<u32, Font>,
}

/// A persistent resource cache for the ggez markedly renderer.
pub struct GgezRivrCache {
    panels: MetroHashMap<PanelId, Canvas>,
    fonts: MetroHashMap<String, FontCache>,

    default_font: Option<String>,
}

impl GgezRivrCache {
    pub fn new() -> Self {
        GgezRivrCache {
            panels: MetroHashMap::default(),
            fonts: MetroHashMap::default(),

            default_font: None,
        }
    }

    /// Adds a font to the cache by its ggez resource path.
    /// This will not actually load the font until it's used with a specific size.
    pub fn add_font<S: Into<String>, P: Into<PathBuf>>(
        &mut self, name: S, location: P
    ) -> Result<(), Error> {
        let name = name.into();

        if self.default_font.is_none() {
            self.default_font = Some(name.clone());
        }

        if self.fonts.contains_key(&name) {
            return Err(Error::Resource(ResourceError::Other {
                resource: Some(name),
                error: "Font already added to cache".into(),
            }))
        }

        self.fonts.insert(name, FontCache {
            path: location.into(),
            sizes: MetroHashMap::default(),
        });

        Ok(())
    }

    pub fn canvas_for_panel(&self, panel_id: PanelId) -> Option<&Canvas> {
        self.panels.get(&panel_id)
    }

    pub fn create_resize_cache(
        &mut self, ctx: &mut Context, panel_id: PanelId, size: Vector2<u32>
    ) -> Result<bool, Error> {
        // If we have a cached canvas and it's of the right size, we only have to clear
        if let Some(canvas) = self.panels.get(&panel_id) {
            if canvas.get_image().width() == size.x &&
                canvas.get_image().height() == size.y {
                return Ok(false)
            }
        }

        // We don't have what we need so create a new canvas
        let canvas = Canvas::new(ctx, size.x, size.y, NumSamples::One).map_err(egtr)?;
        self.panels.insert(panel_id, canvas);

        Ok(true)
    }

    pub fn clear_cache(&mut self, ctx: &mut Context, panel_id: PanelId) -> Result<(), Error> {
        let canvas = self.panels.get(&panel_id).unwrap();
        graphics::set_canvas(ctx, Some(canvas));
        graphics::set_background_color(ctx, (255, 255, 255, 0).into());
        graphics::clear(ctx);

        Ok(())
    }

    pub fn retrive_create_font(
        &mut self, ctx: &mut Context, text_size: u32
    ) -> Result<&Font, Error> {
        // Placeholder
        let text_font: Option<&String> = None;

        // Try to find the font cache, use the default, or error if we can't find it
        let requested_font_name = text_font.or(self.default_font.as_ref())
            .ok_or_else(|| Error::Resource(ResourceError::Other {
                resource: None,
                error: "Could not fall back to default font, no fonts are loaded".into()
            }))?;
        let font_cache = self.fonts.get_mut(requested_font_name)
            .ok_or_else(|| Error::Resource(ResourceError::Other {
                resource: Some(requested_font_name.clone()),
                error: "Font is not in cache".into()
            }))?;

        // Find the cached size for this font, or generate a cache for that
        if !font_cache.sizes.contains_key(&text_size) {
            let font = Font::new(ctx, &font_cache.path, text_size).map_err(egtr)?;
            font_cache.sizes.insert(text_size, font);
        }
        let font = font_cache.sizes.get(&text_size).unwrap();

        Ok(font)
    }
}
