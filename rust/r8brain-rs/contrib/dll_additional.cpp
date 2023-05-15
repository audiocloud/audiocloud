#include <DLL/r8bsrc.h>
#include <CDSPResampler.h>

using namespace r8b;

extern "C"
{
    R8BSRC_DECL int r8b_inlen_for_pos(CR8BResampler const rs, const int ReqOutPos)
    {
        return (((CDSPResampler *)rs)->getInLenBeforeOutPos(ReqOutPos));
    }
}