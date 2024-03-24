use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct ElementTags: u32 {
{% for tag in element_tags %}
        const {{ tag }} = 1 << {{ loop.index0 }};
{% endfor %}
    }
}
