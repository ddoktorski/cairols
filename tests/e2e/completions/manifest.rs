use crate::completions::ManifestCompletion;
use crate::support::insta::test_transform_plain;

#[test]
fn complete_table_header() {
    test_transform_plain!(ManifestCompletion, r#"
        [<caret>]
    "#, @r#"
    caret = """
    [<caret>]
    """

    [[completions]]
    completion_label = ""

    [[completions]]
    completion_label = "cairo"

    [[completions]]
    completion_label = "cairo-plugin"

    [[completions]]
    completion_label = "dependencies"

    [[completions]]
    completion_label = "dev-dependencies"

    [[completions]]
    completion_label = "executable"

    [[completions]]
    completion_label = "features"

    [[completions]]
    completion_label = "lib"

    [[completions]]
    completion_label = "package"

    [[completions]]
    completion_label = "package.authors"

    [[completions]]
    completion_label = "package.cairo-version"

    [[completions]]
    completion_label = "package.description"

    [[completions]]
    completion_label = "package.documentation"

    [[completions]]
    completion_label = "package.edition"

    [[completions]]
    completion_label = "package.homepage"

    [[completions]]
    completion_label = "package.keywords"

    [[completions]]
    completion_label = "package.license"

    [[completions]]
    completion_label = "package.license-file"

    [[completions]]
    completion_label = "package.readme"

    [[completions]]
    completion_label = "package.repository"

    [[completions]]
    completion_label = "package.urls"

    [[completions]]
    completion_label = "package.version"

    [[completions]]
    completion_label = "patch"

    [[completions]]
    completion_label = "profile"

    [[completions]]
    completion_label = "scripts"

    [[completions]]
    completion_label = "target"

    [[completions]]
    completion_label = "target-defaults"

    [[completions]]
    completion_label = "tool"

    [[completions]]
    completion_label = "workspace"

    [[completions]]
    completion_label = "workspace.dependencies"

    [[completions]]
    completion_label = "workspace.package"

    [[completions]]
    completion_label = "workspace.scripts"

    [[completions]]
    completion_label = "workspace.target-defaults"

    [[completions]]
    completion_label = "workspace.tool"
    "#);
}

#[test]
fn complete_package_keys() {
    // Cursor on an empty line in the middle so fixture trim() doesn't remove the cursor line.
    test_transform_plain!(ManifestCompletion, r#"
        [package]
        name = "hello"
        <caret>
        version = "0.1.0"
    "#, @r#"
    caret = """
    <caret>
    """

    [[completions]]
    completion_label = "assets"
    insert_text = "assets = $0"

    [[completions]]
    completion_label = "authors"
    insert_text = "authors = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "authors"
    insert_text = "authors = [$0]"

    [[completions]]
    completion_label = "authors"
    insert_text = "authors = $0"

    [[completions]]
    completion_label = "authors.workspace"
    insert_text = "authors.workspace = ${0:false}"

    [[completions]]
    completion_label = "cairo-version"
    insert_text = "cairo-version = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "cairo-version"
    insert_text = 'cairo-version = "$0"'

    [[completions]]
    completion_label = "cairo-version"
    insert_text = "cairo-version = $0"

    [[completions]]
    completion_label = "cairo-version.workspace"
    insert_text = "cairo-version.workspace = ${0:false}"

    [[completions]]
    completion_label = "description"
    insert_text = "description = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "description"
    insert_text = 'description = "$0"'

    [[completions]]
    completion_label = "description"
    insert_text = "description = $0"

    [[completions]]
    completion_label = "description.workspace"
    insert_text = "description.workspace = ${0:false}"

    [[completions]]
    completion_label = "documentation"
    insert_text = "documentation = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "documentation"
    insert_text = 'documentation = "$0"'

    [[completions]]
    completion_label = "documentation"
    insert_text = "documentation = $0"

    [[completions]]
    completion_label = "documentation.workspace"
    insert_text = "documentation.workspace = ${0:false}"

    [[completions]]
    completion_label = "edition"
    insert_text = "edition = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "edition"
    insert_text = 'edition = "$0"'

    [[completions]]
    completion_label = "edition"
    insert_text = "edition = $0"

    [[completions]]
    completion_label = "edition.workspace"
    insert_text = "edition.workspace = ${0:false}"

    [[completions]]
    completion_label = "experimental-features"
    insert_text = "experimental-features = $0"

    [[completions]]
    completion_label = "homepage"
    insert_text = "homepage = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "homepage"
    insert_text = 'homepage = "$0"'

    [[completions]]
    completion_label = "homepage"
    insert_text = "homepage = $0"

    [[completions]]
    completion_label = "homepage.workspace"
    insert_text = "homepage.workspace = ${0:false}"

    [[completions]]
    completion_label = "include"
    insert_text = "include = $0"

    [[completions]]
    completion_label = "keywords"
    insert_text = "keywords = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "keywords"
    insert_text = "keywords = [$0]"

    [[completions]]
    completion_label = "keywords"
    insert_text = "keywords = $0"

    [[completions]]
    completion_label = "keywords.workspace"
    insert_text = "keywords.workspace = ${0:false}"

    [[completions]]
    completion_label = "license"
    insert_text = "license = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "license"
    insert_text = 'license = "$0"'

    [[completions]]
    completion_label = "license"
    insert_text = "license = $0"

    [[completions]]
    completion_label = "license-file"
    insert_text = "license-file = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "license-file"
    insert_text = 'license-file = "$0"'

    [[completions]]
    completion_label = "license-file"
    insert_text = "license-file = $0"

    [[completions]]
    completion_label = "license-file.workspace"
    insert_text = "license-file.workspace = ${0:false}"

    [[completions]]
    completion_label = "license.workspace"
    insert_text = "license.workspace = ${0:false}"

    [[completions]]
    completion_label = "no-core"
    insert_text = "no-core = $0"

    [[completions]]
    completion_label = "publish"
    insert_text = "publish = $0"

    [[completions]]
    completion_label = "re-export-cairo-plugins"
    insert_text = "re-export-cairo-plugins = $0"

    [[completions]]
    completion_label = "readme"
    insert_text = "readme = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "readme"
    insert_text = 'readme = "$0"'

    [[completions]]
    completion_label = "readme"
    insert_text = "readme = ${0:false}"

    [[completions]]
    completion_label = "readme"
    insert_text = "readme = $0"

    [[completions]]
    completion_label = "readme.workspace"
    insert_text = "readme.workspace = ${0:false}"

    [[completions]]
    completion_label = "repository"
    insert_text = "repository = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "repository"
    insert_text = 'repository = "$0"'

    [[completions]]
    completion_label = "repository"
    insert_text = "repository = $0"

    [[completions]]
    completion_label = "repository.workspace"
    insert_text = "repository.workspace = ${0:false}"

    [[completions]]
    completion_label = "urls"
    insert_text = "urls = $0"

    [[completions]]
    completion_label = "version.workspace"
    insert_text = "version.workspace = ${0:false}"
    "#);
}

#[test]
fn complete_edition_value() {
    test_transform_plain!(ManifestCompletion, r#"
        [package]
        name = "hello"
        version = "0.1.0"
        edition = "<caret>"
    "#, @r#"
    caret = """
    edition = "<caret>"
    """

    [[completions]]
    completion_label = '""'

    [[completions]]
    completion_label = "false"

    [[completions]]
    completion_label = "true"

    [[completions]]
    completion_label = "{ }"
    "#);
}

#[test]
fn complete_dependency_keys() {
    test_transform_plain!(ManifestCompletion, r#"
        [package]
        name = "hello"
        version = "0.1.0"

        [dependencies]
        starknet = { <caret> }
    "#, @r#"
    caret = """
    starknet = { <caret> }
    """

    [[completions]]
    completion_label = "branch"
    insert_text = "branch = $0"

    [[completions]]
    completion_label = "default-features"
    insert_text = "default-features = $0"

    [[completions]]
    completion_label = "features"
    insert_text = "features = $0"

    [[completions]]
    completion_label = "features"
    insert_text = "features = $0"

    [[completions]]
    completion_label = "git"
    insert_text = "git = $0"

    [[completions]]
    completion_label = "path"
    insert_text = 'path = "$0"'

    [[completions]]
    completion_label = "path"
    insert_text = "path = $0"

    [[completions]]
    completion_label = "registry"
    insert_text = "registry = $0"

    [[completions]]
    completion_label = "rev"
    insert_text = "rev = $0"

    [[completions]]
    completion_label = "tag"
    insert_text = "tag = $0"

    [[completions]]
    completion_label = "version"
    insert_text = "version = $0"

    [[completions]]
    completion_label = "workspace"
    insert_text = "workspace = ${0:false}"
    "#);
}

#[test]
fn complete_top_level_empty_line() {
    // Cursor on an empty line at the top level between sections.
    test_transform_plain!(ManifestCompletion, r#"
        [package]
        name = "hello"
        <caret>
        [dependencies]
    "#, @r#"
    caret = """
    <caret>
    """

    [[completions]]
    completion_label = "assets"
    insert_text = "assets = $0"

    [[completions]]
    completion_label = "authors"
    insert_text = "authors = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "authors"
    insert_text = "authors = [$0]"

    [[completions]]
    completion_label = "authors"
    insert_text = "authors = $0"

    [[completions]]
    completion_label = "authors.workspace"
    insert_text = "authors.workspace = ${0:false}"

    [[completions]]
    completion_label = "cairo-version"
    insert_text = "cairo-version = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "cairo-version"
    insert_text = 'cairo-version = "$0"'

    [[completions]]
    completion_label = "cairo-version"
    insert_text = "cairo-version = $0"

    [[completions]]
    completion_label = "cairo-version.workspace"
    insert_text = "cairo-version.workspace = ${0:false}"

    [[completions]]
    completion_label = "description"
    insert_text = "description = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "description"
    insert_text = 'description = "$0"'

    [[completions]]
    completion_label = "description"
    insert_text = "description = $0"

    [[completions]]
    completion_label = "description.workspace"
    insert_text = "description.workspace = ${0:false}"

    [[completions]]
    completion_label = "documentation"
    insert_text = "documentation = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "documentation"
    insert_text = 'documentation = "$0"'

    [[completions]]
    completion_label = "documentation"
    insert_text = "documentation = $0"

    [[completions]]
    completion_label = "documentation.workspace"
    insert_text = "documentation.workspace = ${0:false}"

    [[completions]]
    completion_label = "edition"
    insert_text = "edition = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "edition"
    insert_text = 'edition = "$0"'

    [[completions]]
    completion_label = "edition"
    insert_text = "edition = $0"

    [[completions]]
    completion_label = "edition.workspace"
    insert_text = "edition.workspace = ${0:false}"

    [[completions]]
    completion_label = "experimental-features"
    insert_text = "experimental-features = $0"

    [[completions]]
    completion_label = "homepage"
    insert_text = "homepage = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "homepage"
    insert_text = 'homepage = "$0"'

    [[completions]]
    completion_label = "homepage"
    insert_text = "homepage = $0"

    [[completions]]
    completion_label = "homepage.workspace"
    insert_text = "homepage.workspace = ${0:false}"

    [[completions]]
    completion_label = "include"
    insert_text = "include = $0"

    [[completions]]
    completion_label = "keywords"
    insert_text = "keywords = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "keywords"
    insert_text = "keywords = [$0]"

    [[completions]]
    completion_label = "keywords"
    insert_text = "keywords = $0"

    [[completions]]
    completion_label = "keywords.workspace"
    insert_text = "keywords.workspace = ${0:false}"

    [[completions]]
    completion_label = "license"
    insert_text = "license = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "license"
    insert_text = 'license = "$0"'

    [[completions]]
    completion_label = "license"
    insert_text = "license = $0"

    [[completions]]
    completion_label = "license-file"
    insert_text = "license-file = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "license-file"
    insert_text = 'license-file = "$0"'

    [[completions]]
    completion_label = "license-file"
    insert_text = "license-file = $0"

    [[completions]]
    completion_label = "license-file.workspace"
    insert_text = "license-file.workspace = ${0:false}"

    [[completions]]
    completion_label = "license.workspace"
    insert_text = "license.workspace = ${0:false}"

    [[completions]]
    completion_label = "no-core"
    insert_text = "no-core = $0"

    [[completions]]
    completion_label = "publish"
    insert_text = "publish = $0"

    [[completions]]
    completion_label = "re-export-cairo-plugins"
    insert_text = "re-export-cairo-plugins = $0"

    [[completions]]
    completion_label = "readme"
    insert_text = "readme = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "readme"
    insert_text = 'readme = "$0"'

    [[completions]]
    completion_label = "readme"
    insert_text = "readme = ${0:false}"

    [[completions]]
    completion_label = "readme"
    insert_text = "readme = $0"

    [[completions]]
    completion_label = "readme.workspace"
    insert_text = "readme.workspace = ${0:false}"

    [[completions]]
    completion_label = "repository"
    insert_text = "repository = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "repository"
    insert_text = 'repository = "$0"'

    [[completions]]
    completion_label = "repository"
    insert_text = "repository = $0"

    [[completions]]
    completion_label = "repository.workspace"
    insert_text = "repository.workspace = ${0:false}"

    [[completions]]
    completion_label = "urls"
    insert_text = "urls = $0"

    [[completions]]
    completion_label = "version"
    insert_text = "version = { workspace = ${1:false} }$0"

    [[completions]]
    completion_label = "version"
    insert_text = 'version = "$0"'

    [[completions]]
    completion_label = "version.workspace"
    insert_text = "version.workspace = ${0:false}"
    "#);
}
