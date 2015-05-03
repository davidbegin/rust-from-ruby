require "fiddle"
require "fiddle/import"

module RustPoint
   extend Fiddle::Importer

   dlload "./libpoints.dylib"

   extern "Point* make_point(int, int)"
   # extern "double get_distance(Point*, Point*)"
end

require "./points"
p1 = RustPoint::make_point(10, 10)
p2 = RustPoint::make_point(20, 20)
puts "A Point made in Rust: #{p1.inspect}"

# This produces
# distance = RustPoint::get_distance(p1, p2)
# thread '<unnamed>' panicked at 'arithmetic operation overflowed',
