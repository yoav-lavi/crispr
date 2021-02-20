class Crisper < Formula
  version '0.1.0'
  desc "Scaffold a project from a template"
  homepage "https://github.com/yoav-lavi/crispr"

  if OS.mac?
      url "https://github.com/yoav-lavi/crispr/releases/download/v#{version}/crispr-v#{version}-x86_64-apple-darwin.tag.gz"
      sha256 "13fb8f8f2366b7ba566be395935e46a105c1341fa1a62167f3a91e91c497a8a2"
  end

  conflicts_with "crispr"

  def install
    bin.install "crispr"
  end
end
