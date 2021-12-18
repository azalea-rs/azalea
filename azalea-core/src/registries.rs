struct RegistryHolder {}

fn make_network_codec() -> NetworkCodec {
    // Codec codec = ResourceLocation.CODEC.xmap(ResourceKey::createRegistryKey, ResourceKey::location);
    // Codec codec2 = codec.partialDispatch("type", mappedRegistry -> DataResult.success(mappedRegistry.key()), resourceKey -> RegistryHolder.getNetworkCodec(resourceKey).map(codec -> MappedRegistry.networkCodec(resourceKey, Lifecycle.experimental(), codec)));
    // UnboundedMapCodec unboundedMapCodec = Codec.unboundedMap((Codec)codec, (Codec)codec2);
    // return RegistryHolder.captureMap(unboundedMapCodec);
}
