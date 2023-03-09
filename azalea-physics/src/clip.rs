use azalea_core::{BlockPos, Vec3};

//    static <T, C> T traverseBlocks(Vec3 from, Vec3 to, C context,
// BiFunction<C, BlockPos, T> getHitResult,     Function<C, T> getMissResult) {
//  if (from.equals(to)) {
//     return getMissResult.apply(context);
//  } else {
//     double rightAfterEndX = Mth.lerp(-1.0E-7D, to.x, from.x); // var5
//     double rightAfterEndY = Mth.lerp(-1.0E-7D, to.y, from.y); // var7
//     double rightAfterEndZ = Mth.lerp(-1.0E-7D, to.z, from.z); // var9
//     double rightBeforeStartX = Mth.lerp(-1.0E-7D, from.x, to.x); // var11
//     double rightBeforeStartY = Mth.lerp(-1.0E-7D, from.y, to.y); // var13
//     double rightBeforeStartZ = Mth.lerp(-1.0E-7D, from.z, to.z); // var15
//     int currentBlockX = Mth.floor(rightBeforeStartX); // var17
//     int currentBlockY = Mth.floor(rightBeforeStartY); // var18
//     int currentBlockZ = Mth.floor(rightBeforeStartZ); // var19
//     BlockPos.MutableBlockPos blockPos = new
// BlockPos.MutableBlockPos(currentBlockX, currentBlockY, currentBlockZ);
//     Object data = getHitResult.apply(context, blockPos);
//     if (data != null) {
//        return data;
//     } else {
//        double vectorX = rightAfterEndX - rightBeforeStartX; // var22
//        double vectorY = rightAfterEndY - rightBeforeStartX; // var24
//        double vectorZ = rightAfterEndZ - rightBeforeStartX; // var26
//        int vectorXSign = Mth.sign(vectorX); // var28
//        int vectorYSign = Mth.sign(vectorY); // var29
//        int vectorZSign = Mth.sign(vectorZ); // var30
//        double percentageStepX = vectorXSign == 0 ? 1.7976931348623157E308D :
// ((double) vectorXSign / vectorX); // var31        double percentageStepY =
// vectorYSign == 0 ? 1.7976931348623157E308D : ((double) vectorYSign /
// vectorY); // var33        double percentageStepZ = vectorZSign == 0 ?
// 1.7976931348623157E308D : ((double) vectorZSign / vectorZ); // var35
//        double xPercentage = percentageStepX * (vectorXSign > 0 ? 1.0D -
// Mth.frac(rightBeforeStartX) : Mth.frac(rightBeforeStartY)); // var37
//        double yPercentage = percentageStepY * (vectorYSign > 0 ? 1.0D -
// Mth.frac(rightBeforeStartY) : Mth.frac(rightBeforeStartX)); // var39
//        double zPercentage = percentageStepZ * (vectorZSign > 0 ? 1.0D -
// Mth.frac(rightBeforeStartZ) : Mth.frac(rightBeforeStartZ)); // var41

//        Object data;
//        do {
//           if (xPercentage > 1.0D && yPercentage > 1.0D && zPercentage > 1.0D)
// {              return getMissResult.apply(context);
//           }

//           if (xPercentage < yPercentage) {
//              if (xPercentage < zPercentage) {
//                 currentBlockX += vectorXSign;
//                 xPercentage += percentageStepX;
//              } else {
//                 currentBlockZ += vectorZSign;
//                 zPercentage += percentageStepZ;
//              }
//           } else if (posY < posZ) {
//              currentBlockY += vectorYSign;
//              yPercentage += percentageStepY;
//           } else {
//              currentBlockZ += vectorZSign;
//              zPercentage += percentageStepZ;
//           }

//           data = getHitResult.apply(context, blockPos.set(currentBlockX,
// currentBlockY, currentBlockZ));        } while (data == null);

//        return data;
//     }
//  }
// }

//    static <T, C> T traverseBlocks(Vec3 from, Vec3 to, C context,
// BiFunction<C, BlockPos, T> getHitResult,     Function<C, T> getMissResult) {
pub fn traverse_blocks<C, T>(
    from: Vec3,
    to: Vec3,
    context: C,
    get_hit_result: fn(C, BlockPos) -> T,
    get_miss_result: fn(C) -> T,
) {
    //  if (from.equals(to)) {
    //     return getHitResult.apply(context);
    //  } else {
    //     double rightAfterEndX = Mth.lerp(-1.0E-7D, to.x, from.x); // var5
    //     double rightAfterEndY = Mth.lerp(-1.0E-7D, to.y, from.y); // var7
    //     double rightAfterEndZ = Mth.lerp(-1.0E-7D, to.z, from.z); // var9
    //     double rightBeforeStartX = Mth.lerp(-1.0E-7D, from.x, to.x); // var11
    //     double rightBeforeStartY = Mth.lerp(-1.0E-7D, from.y, to.y); // var13
    //     double rightBeforeStartZ = Mth.lerp(-1.0E-7D, from.z, to.z); // var15

    if from == to {
        return get_miss_result(context);
    }

    let right_after_end = Vec3 {
        
    };
}
