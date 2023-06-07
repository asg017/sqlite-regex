require "version"

module SqliteRegex
  class Error < StandardError; end
  def self.regex_loadable_path
    File.expand_path('../regex0', __FILE__)
  end
  def self.load(db)
    db.load_extension(self.regex_loadable_path)
  end
end
