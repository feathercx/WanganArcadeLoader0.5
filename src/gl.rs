use crate::*;
use glfw::*;
use std::ffi::CString;

const FUNCS: [&'static str; 1110] = [
	"glUniform4iARB",
	"glTexSubImage3D",
	"glDrawElements",
	"glSetFenceNV",
	"glCompressedTexImage3D",
	"glTexCoordPointer",
	"glUniform2i",
	"glTexCoord1hNV",
	"glMultTransformEXT",
	"glTexGeniv",
	"glGetVertexAttribfvARB",
	"glColorTableParameteriv",
	"glMultiTexCoord4hNV",
	"glColor3ub",
	"glIsTexture",
	"glNewList",
	"glMultTransposeMatrixf",
	"glUnlockArraysEXT",
	"glCompressedTexImage1D",
	"glTestFenceNV",
	"glVertexAttrib4sARB",
	"glUniformMatrix2fv",
	"glRasterPos2s",
	"glFogfv",
	"glIndexub",
	"glIndexi",
	"glMultiTexCoord4fvSGIS",
	"glVertexAttrib4dNV",
	"glColorTableParameterfv",
	"glColor4ubv",
	"glCompressedTexImage2D",
	"glMultiTexCoord2ivSGIS",
	"glProgramLocalParameters4fvNV",
	"glRasterPos2fv",
	"glPixelStorei",
	"glMultiTexCoordPointerSGIS",
	"glGetPixelMapusv",
	"glGetProgramivARB",
	"glHint",
	"glWindowPos2f",
	"glUniform1ivARB",
	"glEvalPoint2",
	"glBitmap",
	"glAreTexturesResident",
	"glGetProgramLocalParameterdvARB",
	"glGetAttribLocationARB",
	"glSecondaryColor3ui",
	"glGetAttribLocation",
	"glVertexAttrib4NubARB",
	"glVertexAttrib4ivARB",
	"glVertexAttrib3dARB",
	"glMultiTexCoord2dvARB",
	"glSecondaryColor3bvEXT",
	"glMapBufferARB",
	"glFogf",
	"glMultMatrixd",
	"glMapBuffer",
	"glPointParameterfvEXT",
	"glTexCoord1sv",
	"glRectsv",
	"glMultiTexCoord1svARB",
	"glPointParameterfv",
	"glNormal3hNV",
	"glVertexAttrib4Nsv",
	"glGetColorTableParameterivEXT",
	"glPopAttrib",
	"glColor3bv",
	"glPopClientAttrib",
	"glVertexAttrib1dARB",
	"glSecondaryColorPointer",
	"glTexImage2D",
	"glVertexAttrib4usvARB",
	"glGetVertexAttribdvARB",
	"glVertexAttrib4hNV",
	"glWindowPos3sARB",
	"glReadPixels",
	"glVertexAttrib4hvNV",
	"glTexCoord1f",
	"glFogCoordhNV",
	"glUniform2iv",
	"glGetLightiv",
	"glDeleteProgramsNV",
	"glColorPointerEXT",
	"glTexCoord4f",
	"glFogCoordfEXT",
	"glGenArraySetsARB",
	"glVertexAttrib1hNV",
	"glAccum",
	"glGetBufferSubData",
	"glDrawPixels",
	"glMultiTexCoord4fARB",
	"glFinalCombinerInputNV",
	"glVertex4f",
	"glDrawBuffersATI",
	"glGenTextures",
	"glExecuteProgramNV",
	"glGetUniformfv",
	"glMultiTexCoord1iv",
	"glProgramParameter4dvNV",
	"glRectdv",
	"glMultiTexCoord1hNV",
	"glMapGrid2f",
	"glProgramEnvParameter4dARB",
	"glVertexAttrib1svNV",
	"glBindProgramNV",
	"glGetProgramLocalParameterfvARB",
	"glEndOcclusionQueryNV",
	"glExtensions",
	"glTexImage3D",
	"glGetColorTableParameterfvEXT",
	"glSecondaryColor3iEXT",
	"glValidateProgramARB",
	"glVertexAttrib4uiv",
	"glVertex4s",
	"glWindowPos3dv",
	"glTexEnvfv",
	"glClipPlane",
	"glFinishFenceNV",
	"glVertexAttrib1fARB",
	"glCompressedTexImage2DARB",
	"glMultiTexCoord1iARB",
	"glGetQueryivARB",
	"glStencilFuncSeparate",
	"glTexParameteriv",
	"glMultiTexCoord1dvARB",
	"glRasterPos3f",
	"glUniformMatrix3fv",
	"glColor3us",
	"glWindowPos2i",
	"glColor3ubv",
	"glMultiTexCoord2fARB",
	"glVertex2hNV",
	"glColorSubTableEXT",
	"glGetProgramEnvParameterfvARB",
	"glDrawRangeElements",
	"glMapGrid1f",
	"glVertex3d",
	"glColor3f",
	"glVertexAttrib4fvNV",
	"glVertexAttrib1f",
	"glPointParameteriNV",
	"glMultiTexCoord1dv",
	"glSecondaryColor3fvEXT",
	"glPushClientAttrib",
	"glAlphaFunc",
	"glGetDoublev",
	"glUniform3fvARB",
	"glGenFramebuffersEXT",
	"glMultiTexCoord3hNV",
	"glVertexAttrib4NubvARB",
	"glMapGrid2d",
	"glIndexMask",
	"glMultiTexCoord3i",
	"glTexCoord3s",
	"glConvolutionFilter2D",
	"glColor4ub",
	"glVertexAttribs4dvNV",
	"glUniformMatrix2fvARB",
	"glColor3usv",
	"glNormal3b",
	"glBlendColor",
	"glRotatef",
	"glMultiTexCoord3dv",
	"glVertex4sv",
	"glGetObjectParameterfvARB",
	"glBeginQueryARB",
	"glClampColorARB",
	"glStencilFunc",
	"glVertexAttrib4fvARB",
	"glTexImage3DEXT",
	"glTexCoord2d",
	"glProgramNamedParameter4dNV",
	"glValidBackBufferHintAutodesk",
	"glVertexAttrib1sARB",
	"glCombinerParameterivNV",
	"glCopyPixels",
	"glTexCoord1s",
	"glCompressedTexImage1DARB",
	"glEvalCoord1f",
	"glPointParameteriv",
	"glVertexAttribs2fvNV",
	"glFlush",
	"glMultiTexCoord1ivARB",
	"glEvalMesh2",
	"glMultiTexCoord2iv",
	"glSecondaryColorPointerEXT",
	"glColorTableEXT",
	"glGetProgramivNV",
	"glRasterPos2d",
	"glCompileShader",
	"glListBase",
	"glIndexf",
	"glGetBufferParameterivARB",
	"glWindowPos3fv",
	"glRasterPos2dv",
	"glSecondaryColor3i",
	"glUseProgramObjectARB",
	"glVertexAttrib4NsvARB",
	"glMultiTexCoord2fvSGIS",
	"glTexCoord3d",
	"glPassThrough",
	"glFinish",
	"glVertexAttrib3s",
	"glVertexAttribs2dvNV",
	"glMultiTexCoord1fvSGIS",
	"glMultTransposeMatrixfARB",
	"glIsProgramARB",
	"glMultiTexCoord4iSGIS",
	"glVertexAttrib4s",
	"glEndQueryARB",
	"glMultiTexCoord3svARB",
	"glMultiTexCoord1i",
	"glGetPixelMapfv",
	"glVertexAttrib3fNV",
	"glMultiTexCoord1s",
	"glLoadTransposeMatrixfARB",
	"glUniform4fv",
	"glMultiTexCoord1d",
	"glEnableClientState",
	"glNormal3s",
	"glHistogram",
	"glNormal3dv",
	"glBlendFuncSeparate",
	"glMultiTexCoord2sSGIS",
	"glVertex3f",
	"glGetBufferPointervARB",
	"glUseProgram",
	"glLoadProgramNV",
	"glRequestResidentProgramsNV",
	"glWindowPos2fvARB",
	"glVertexAttrib2fvARB",
	"glVertexAttrib4ubvARB",
	"glIndexsv",
	"glMultiTexCoord4dARB",
	"glVertexAttrib2sARB",
	"glProgramLocalParameter4dvARB",
	"glGetQueryObjecti64vEXT",
	"glVertex4i",
	"glVertexAttrib2dv",
	"glRasterPos4fv",
	"glEndConditionalRenderNVX",
	"glMultiTexCoord4ivARB",
	"glEnableVertexAttribArray",
	"glMultiTexCoord1dvSGIS",
	"glWindowPos3svARB",
	"glIsQuery",
	"glVertexAttrib3sARB",
	"glPrimitiveRestartIndexNV",
	"glSecondaryColor3fv",
	"glMateriali",
	"glNormal3fv",
	"glGetBufferSubDataARB",
	"glGetQueryiv",
	"glTexGend",
	"glLightModeliv",
	"glTexCoord3i",
	"glMultiTexCoord2d",
	"glShaderSource",
	"glVertexAttrib4f",
	"glCopyTexImage2D",
	"glDeleteProgramsARB",
	"glSecondaryColor3iv",
	"glTexCoord1i",
	"glPixelMapfv",
	"glVertexAttrib1s",
	"glProgramParameters4dvNV",
	"glConvolutionParameteri",
	"glCopyConvolutionFilter1D",
	"glTexImage1D",
	"glRasterPos3i",
	"glMultiTexCoord1svSGIS",
	"glVertexAttrib2d",
	"glVertexAttrib4fNV",
	"glMultiTexCoord2fv",
	"glVertex4dv",
	"glSeparableFilter2D",
	"glUniformMatrix3fvARB",
	"glGetVertexAttribdvARB",
	"glCopyTexSubImage1D",
	"glTexCoord4s",
	"glVertex3hvNV",
	"glCombinerOutputNV",
	"glDrawArrays",
	"glProgramEnvParameter4dvARB",
	"glTexCoord2fv",
	"glColor3s",
	"glWindowPos2d",
	"glMultMatrixf",
	"glDrawBuffersARB",
	"glIsTextureEXT",
	"glTexCoord4sv",
	"glCopyConvolutionFilter2D",
	"glClearStencil",
	"glUniform4ivARB",
	"glGetProgramEnvParameterdvARB",
	"glGetActiveUniformARB",
	"glWindowPos3i",
	"glGetVertexAttribdv",
	"glSecondaryColor3us",
	"glSecondaryColor3svEXT",
	"glTexGeni",
	"glBlendEquationSeparate",
	"glUniform2fvARB",
	"glVertexAttrib4NivARB",
	"glColor4hNV",
	"glUniform3f",
	"glCompressedTexSubImage3D",
	"glVertexAttrib4NusvARB",
	"glWindowPos3d",
	"glVertexAttrib4svARB",
	"glUniform4fARB",
	"glVertexAttrib3dARB",
	"glMultiTexCoord3ivSGIS",
	"glCombinerInputNV",
	"glDeleteProgramsARB",
	"glGetTexLevelParameteriv",
	"glRasterPos2f",
	"glMultiTexCoord4dvSGIS",
	"glVertexAttrib2hNV",
	"glMultiTexCoord1sSGIS",
	"glTexParameterf",
	"glVertexAttrib3dNV",
	"glVertexAttrib2fvNV",
	"glWindowPos2s",
	"glDisable",
	"glGetPointervEXT",
	"glMultiTexCoord2svARB",
	"glDepthMask",
	"glGetMapdv",
	"glVertexAttrib4ivARB",
	"glProgramEnvParameters4fvNV",
	"glEvalCoord2d",
	"glUniform2f",
	"glVertexAttrib1fARB",
	"glVertexAttrib1svARB",
	"glGetCombinerInputParameterfvNV",
	"glSelectBuffer",
	"glMatrixMode",
	"glColor4b",
	"glGenProgramsARB",
	"glBufferData",
	"glDeleteProgram",
	"glNormal3iv",
	"glVertexAttrib1fvARB",
	"glGetProgramStringARB",
	"glProgramNamedParameter4dvNV",
	"glSecondaryColor3uiEXT",
	"glSecondaryColor3sv",
	"glColorMask",
	"glTexCoord1d",
	"glBindAttribLocation",
	"glGetVertexAttribivNV",
	"glSecondaryColor3bv",
	"glDrawBuffers",
	"glFogCoorddv",
	"glVertexAttrib2svARB",
	"glStencilClearTagEXT",
	"glBindTextureEXT",
	"glVertexAttrib3d",
	"glScaled",
	"glGetRenderbufferParameterivEXT",
	"glGetColorTable",
	"glMultiTexCoord2sARB",
	"glProgramParameter4fvNV",
	"glGetCombinerOutputParameterfvNV",
	"glBindFramebufferEXT",
	"glMultiDrawArraysEXT",
	"glBlendFuncSeparateEXT",
	"glPushName",
	"glVertexAttribs1hvNV",
	"glGetTexGenfv",
	"glMultiTexCoord2dv",
	"glFogCoordPointerEXT",
	"glVertexAttrib4Nusv",
	"glProgramEnvParameter4fARB",
	"glRasterPos4i",
	"glVertexAttrib1sNV",
	"glMultiTexCoord3fvSGIS",
	"glGetMaterialiv",
	"glRasterPos4f",
	"glRectiv",
	"glMultiDrawElementsEXT",
	"glRecti",
	"glCombinerParameterfvNV",
	"glVertex2iv",
	"glProgramParameter4dNV",
	"glVertexAttrib4NuivARB",
	"glMultiTexCoord3fv",
	"glMinmax",
	"glAttachObjectARB",
	"glGetProgramStringNV",
	"glMultiTexCoord4iv",
	"glViewport",
	"glVertexAttribPointerNV",
	"glVertexAttrib3hvNV",
	"glBeginQuery",
	"glGetProgramStringARB",
	"glVertexAttrib4Nuiv",
	"glVertex3fv",
	"glRasterPos4d",
	"glVertexAttribs3dvNV",
	"glGetCompressedTexImageARB",
	"glBegin",
	"glIsProgram",
	"glTexCoord2dv",
	"glVertexAttrib1svARB",
	"glVertexAttrib4dv",
	"glRasterPos4iv",
	"glMultiTexCoord2fvARB",
	"glFogCoorddEXT",
	"glTexCoord4fv",
	"glVertexAttrib4ubvNV",
	"glGetUniformfvARB",
	"glVertexAttrib4svARB",
	"glGetMinmaxParameterfv",
	"glMap1f",
	"glMultiTexCoord2hNV",
	"glPolygonStipple",
	"glVertexAttrib1dv",
	"glMultiTexCoord3dSGIS",
	"glEvalCoord1fv",
	"glScissor",
	"glGetQueryObjectiv",
	"glGetProgramParameterdvNV",
	"glVertexAttribPointer",
	"glVertexAttrib4bvARB",
	"glMultiTexCoord2svSGIS",
	"glGetTexParameterfv",
	"glDrawBuffer",
	"glProgramEnvParameter4fvARB",
	"glDeleteRenderbuffersEXT",
	"glLineWidth",
	"glInitNames",
	"glVertexAttrib4NubARB",
	"glSecondaryColor3usv",
	"glVertexPointerEXT",
	"glActiveStencilFaceEXT",
	"glEdgeFlagPointer",
	"glUniform1fvARB",
	"glMultiTexCoord4sSGIS",
	"glDeleteBuffersARB",
	"glProgramLocalParameter4dARB",
	"glCopyTexImage1D",
	"glCopyColorSubTable",
	"glMultiTexCoord3iARB",
	"glGetBufferParameteriv",
	"glVertex4d",
	"glGetSeparableFilter",
	"glTexCoord3hNV",
	"glMultiTexCoord2dvSGIS",
	"glMultiTexCoord4hvNV",
	"glDeleteTexturesEXT",
	"glSecondaryColor3d",
	"glProgramStringARB",
	"glCopyTexSubImage3DEXT",
	"glVertexAttrib2f",
	"glGetHistogramParameteriv",
	"glVertexAttrib2s",
	"glMultiTexCoord1sARB",
	"glMultiTexCoord4fSGIS",
	"glPolygonOffset",
	"glUniform3fARB",
	"glColor4uiv",
	"glGetTexLevelParameterfv",
	"glMultiTexCoord1iSGIS",
	"glRasterPos4s",
	"glUniform3iARB",
	"glTexCoord4iv",
	"glSecondaryColor3ivEXT",
	"glEvalPoint1",
	"glWindowBackBufferHintAutodesk",
	"glResetMinmax",
	"glIsFramebufferEXT",
	"glColor4hvNV",
	"glCompressedTexSubImage2D",
	"glGetVertexAttribdvNV",
	"glRasterPos4sv",
	"glMultiTexCoord1dSGIS",
	"glColor4iv",
	"glRasterPos3sv",
	"glVertexAttrib2dvNV",
	"glEnableVertexAttribArrayARB",
	"glIndexd",
	"glVertexAttrib4Nub",
	"glUniform3ivARB",
	"glWindowPos2sARB",
	"glUniform3fv",
	"glDepthRange",
	"glColor4fv",
	"glGetCompressedTexImage",
	"glClientActiveTextureARB",
	"glTexCoordPointerEXT",
	"glGetQueryObjectuiv",
	"glDeleteOcclusionQueriesNV",
	"glBlendEquationEXT",
	"glVertexAttribPointerARB",
	"glVertex2i",
	"glWindowPos2iARB",
	"glGetVertexAttribfv",
	"glFlushPixelDataRangeNV",
	"glMultiTexCoord3hvNV",
	"glGetFloatv",
	"glPixelDataRangeNV",
	"glGetPolygonStipple",
	"glMultiTexCoord4sARB",
	"glNormal3sv",
	"glGetProgramLocalParameterfvARB",
	"glMultiTexCoord4ivSGIS",
	"glGetUniformivARB",
	"glVertex4hvNV",
	"glWindowPos3dvARB",
	"glConvolutionParameteriv",
	"glRasterPos2sv",
	"glGetVertexAttribfvARB",
	"glCopyTexSubImage2D",
	"glSecondaryColor3dvEXT",
	"glVertexAttrib4NsvARB",
	"glVertexAttrib2fARB",
	"glBufferDataARB",
	"glGetAttachedShaders",
	"glTexGenf",
	"glWindowPos3fARB",
	"glPixelTransferf",
	"glMultiTexCoord2iSGIS",
	"glSecondaryColor3dv",
	"glVertex2f",
	"glPixelZoom",
	"glFramebufferTexture1DEXT",
	"glMultiTexCoord2f",
	"glLightiv",
	"glUniform1fARB",
	"glVertexAttribPointerARB",
	"glStencilMask",
	"glTexParameteri",
	"glMultiTexCoord4s",
	"glTexCoord4i",
	"glColor4sv",
	"glVertexAttrib2sv",
	"glRasterPos2i",
	"glIsProgramNV",
	"glProgramLocalParameter4fvARB",
	"glSecondaryColor3hNV",
	"glWindowPos2svARB",
	"glGetQueryObjectuivARB",
	"glGenRenderbuffersEXT",
	"glGetFinalCombinerInputParameterfvNV",
	"glGetMinmax",
	"glMultiTexCoord4dvARB",
	"glColor3uiv",
	"glFramebufferTexture3DEXT",
	"glVertexAttrib4bv",
	"glUniform1i",
	"glIndexfv",
	"glMultiTexCoord3d",
	"glMultiTexCoord2fSGIS",
	"glDetachShader",
	"glEvalCoord1dv",
	"glVertexAttrib4uivARB",
	"glCheckFramebufferStatusEXT",
	"glBindBufferARB",
	"glPointParameterivNV",
	"glGetTexImage",
	"glVertexAttrib4NuivARB",
	"glVertexAttrib2svNV",
	"glMultiTexCoord3fARB",
	"glShaderSourceARB",
	"glVertexAttrib2svARB",
	"glVertexAttrib4bvARB",
	"glGetObjectParameterivARB",
	"glTexCoord2f",
	"glGenLists",
	"glMultiDrawElements",
	"glSecondaryColor3usEXT",
	"glLoadMatrixf",
	"glSecondaryColor3usvEXT",
	"glGetHistogramParameterfv",
	"glTexCoord1hvNV",
	"glMultiTexCoord3iSGIS",
	"glBindProgramARB",
	"glMap1d",
	"glMultiTexCoord3svSGIS",
	"glDeleteFramebuffersEXT",
	"glCombinerParameteriNV",
	"glMultiTexCoord2dARB",
	"glPointParameterf",
	"glProgramLocalParameter4fARB",
	"glVertexAttribs3fvNV",
	"glPixelStoref",
	"glRasterPos3fv",
	"glDeleteQueriesARB",
	"glMultiTexCoord2hvNV",
	"glWindowPos3s",
	"glTexCoord2i",
	"glTexCoord3f",
	"glTexSubImage3DEXT",
	"glMultiTexCoord3dvSGIS",
	"glGetProgramEnvParameterfvARB",
	"glLighti",
	"glLightf",
	"glSelectTextureCoordSetSGIS",
	"glSecondaryColor3ubvEXT",
	"glCompressedTexSubImage1D",
	"glVertexAttrib3dvARB",
	"glMultiTexCoord1f",
	"glProgramLocalParameter4fvARB",
	"glResetHistogram",
	"glMultiTexCoord2s",
	"glColorMaterial",
	"glGenFencesNV",
	"glVertexAttrib1fNV",
	"glClearAccum",
	"glGetMapiv",
	"glPointParameterfARB",
	"glGetProgramNamedParameterdvNV",
	"glUniform1iARB",
	"glStencilOpSeparate",
	"glProgramEnvParameter4dvARB",
	"glSecondaryColor3uiv",
	"glGetHistogram",
	"glGetQueryObjectivARB",
	"glCallList",
	"glUniform1iv",
	"glMultiTexCoord2sv",
	"glTexCoord3sv",
	"glLightfv",
	"glGetError",
	"glVertexAttrib2fNV",
	"glGetMinmaxParameteriv",
	"glProgramLocalParameter4dvARB",
	"glDeleteFencesNV",
	"glGetIntegerv",
	"glFogCoordhvNV",
	"glGetTexEnvfv",
	"glSecondaryColor3s",
	"glVertexAttrib1sv",
	"glRectf",
	"glAttachShader",
	"glGetTexEnviv",
	"glUniform4i",
	"glVertexAttrib2sARB",
	"glPushAttrib",
	"glVertexAttrib4ubv",
	"glDrawArraysEXT",
	"glRenderMode",
	"glCallLists",
	"glVertex3sv",
	"glMultiTexCoord3fvARB",
	"glVertexAttrib3fvARB",
	"glLineStipple",
	"glVertexAttrib2dvARB",
	"glMultiTexCoord3s",
	"glVertexAttrib2hvNV",
	"glTexGenfv",
	"glVertexAttribs1svNV",
	"glMultiTexCoord1sv",
	"glColor4bv",
	"glLoadIdentity",
	"glMultiTexCoord1fARB",
	"glCompressedTexSubImage1DARB",
	"glVertexAttrib3sNV",
	"glDrawMeshNV",
	"glUniform4fvARB",
	"glGetTexParameteriv",
	"glIsBufferARB",
	"glUniformMatrix4fv",
	"glIndexiv",
	"glPointSize",
	"glGetShaderiv",
	"glPrioritizeTexturesEXT",
	"glGetColorTableParameteriv",
	"glPixelTransferi",
	"glFogCoordPointer",
	"glUniform2fv",
	"glTexEnviv",
	"glMultiTexCoord3fSGIS",
	"glGetShaderSourceARB",
	"glTranslated",
	"glPopMatrix",
	"glSecondaryColor3sEXT",
	"glGetInfoLogARB",
	"glIndexPointer",
	"glLinkProgram",
	"glGetVertexAttribPointervARB",
	"glTexCoord1fv",
	"glMultiTexCoord4f",
	"glTexCoord1dv",
	"glEvalCoord2dv",
	"glVertexPointer",
	"glGetOcclusionQueryivNV",
	"glVertexAttrib3dv",
	"glPrioritizeTextures",
	"glSecondaryColor3ubEXT",
	"glGenBuffersARB",
	"glColor3dv",
	"glTexCoord2s",
	"glWindowPos3dARB",
	"glPointParameterfvARB",
	"glProgramParameters4fvNV",
	"glGetColorTableParameterfv",
	"glGetVertexAttribPointervARB",
	"glVertexAttrib4fARB",
	"glVertexAttrib4iv",
	"glVertexAttrib2dARB",
	"glMultiTexCoord4i",
	"glTrackMatrixNV",
	"glWindowPos3iARB",
	"glFogCoordfv",
	"glVertex2hvNV",
	"glGetBufferPointerv",
	"glGetConvolutionFilter",
	"glVertexAttrib2sNV",
	"glWindowPos3fvARB",
	"glRenderbufferStorageEXT",
	"glMultiTexCoord2iARB",
	"glGetBooleanv",
	"glWindowPos2dARB",
	"glRasterPos4dv",
	"glProgramNamedParameter4fvNV",
	"glVertexAttrib3svARB",
	"glGetActiveUniform",
	"glWindowPos3sv",
	"glVertexAttrib4usv",
	"glVertexAttrib4dARB",
	"glColor3i",
	"glOrtho",
	"glGetHandleARB",
	"glVertexAttrib1fv",
	"glGetFinalCombinerInputParameterivNV",
	"glMultiTexCoord4svSGIS",
	"glProgramParameter4fNV",
	"glVertexAttrib3hNV",
	"glVertexAttrib4fv",
	"glColor3hNV",
	"glRasterPos3iv",
	"glFogi",
	"glClearColor",
	"glVertexAttrib3f",
	"glColor4usv",
	"glUniform2iARB",
	"glTexCoord4hvNV",
	"glRasterPos3dv",
	"glVertexAttrib2fARB",
	"glUnmapBufferARB",
	"glMultiTexCoord4dv",
	"glGetMapfv",
	"glMap2d",
	"glGenProgramsNV",
	"glVertexAttrib4NusvARB",
	"glVertex2sv",
	"glVertexAttrib1hvNV",
	"glValidateProgram",
	"glTexSubImage2D",
	"glFrontFace",
	"glRasterPos3d",
	"glMultTransposeMatrixd",
	"glVertexAttrib3sARB",
	"glColor3iv",
	"glDeleteObjectARB",
	"glLinkProgramARB",
	"glLoadMatrixd",
	"glMultiTexCoord4sv",
	"glGetUniformLocation",
	"glRectfv",
	"glGetLightfv",
	"glWindowPos2dv",
	"glVertexAttrib3sv",
	"glTexCoord1iv",
	"glVertexAttrib4NubvARB",
	"glTexCoord4d",
	"glGetProgramNamedParameterfvNV",
	"glVertexAttrib4dvARB",
	"glMultiTexCoord1fSGIS",
	"glColor3ui",
	"glTexGendv",
	"glVertexArrayRangeNV",
	"glGetVertexAttribiv",
	"glGetProgramInfoLog",
	"glMultiTexCoord3dARB",
	"glTexCoord3fv",
	"glGetFenceivNV",
	"glVertexAttrib1dARB",
	"glMultiTexCoord1fvARB",
	"glStencilMaskSeparate",
	"glVertex3hNV",
	"glWindowPos2iv",
	"glClearIndex",
	"glGetActiveAttrib",
	"glMultiTexCoord1ivSGIS",
	"glDepthFunc",
	"glWindowPos2ivARB",
	"glColorSubTable",
	"glGetString",
	"glCreateShader",
	"glUniform2fARB",
	"glVertexAttrib4sv",
	"glArrayElement",
	"glInterleavedArrays",
	"glSecondaryColor3f",
	"glGetClipPlane",
	"glNormalPointerEXT",
	"glGetVertexAttribivARB",
	"glPrimitiveRestartNV",
	"glEndList",
	"glUniform2ivARB",
	"glMaterialf",
	"glVertexAttribs1dvNV",
	"glCreateProgram",
	"glSecondaryColor3bEXT",
	"glTexCoord2hvNV",
	"glLightModeli",
	"glIsRenderbufferEXT",
	"glError",
	"glTexParameterfv",
	"glVertexAttrib1dvARB",
	"glGetAttachedObjectsARB",
	"glVertexAttribs3hvNV",
	"glTexCoord4dv",
	"glMultiTexCoord1fv",
	"glMultiTexCoord2ivARB",
	"glBindAttribLocationARB",
	"glVertexAttribs2svNV",
	"glConvolutionFilter1D",
	"glMultiTexCoord1dARB",
	"glBufferSubData",
	"glVertexAttribs4svNV",
	"glTexEnvf",
	"glBeginOcclusionQueryNV",
	"glBindRenderbufferEXT",
	"glProgramNamedParameter4fNV",
	"glPushMatrix",
	"glVertex2s",
	"glVertexAttrib2fv",
	"glDeleteQueries",
	"glEnableVertexAttribArrayARB",
	"glRotated",
	"glMaterialiv",
	"glCopyColorTable",
	"glUniform3i",
	"glVertexAttrib4dARB",
	"glFogCoorddvEXT",
	"glIsQueryARB",
	"glIsArraySetARB",
	"glLoadTransposeMatrixf",
	"glBlendColorEXT",
	"glSecondaryColor3b",
	"glGetMaterialfv",
	"glBindTexture",
	"glWindowPos3iv",
	"glGetColorTableEXT",
	"glEnable",
	"glColor4us",
	"glVertexAttrib4fvARB",
	"glNormal3f",
	"glPixelMapuiv",
	"glFogCoordd",
	"glCullFace",
	"glDeleteBuffers",
	"glWindowPos2fv",
	"glNormal3hvNV",
	"glIsBuffer",
	"glPointParameterfv",
	"glBindProgramARB",
	"glMultiTexCoord4fvARB",
	"glWindowPos2fARB",
	"glProgramLocalParameter4dARB",
	"glGetTexGeniv",
	"glGetTrackMatrixivNV",
	"glConvolutionParameterf",
	"glTexCoord2iv",
	"glUnmapBuffer",
	"glGetQueryObjectui64vEXT",
	"glMultiTexCoord4dSGIS",
	"glMultiTexCoord4svARB",
	"glMultiTexCoord3sv",
	"glProgramStringARB",
	"glCompressedTexImage3DARB",
	"glColor4ui",
	"glSecondaryColor3hvNV",
	"glFrustum",
	"glMultTransposeMatrixdARB",
	"glVertexAttrib4sNV",
	"glGetProgramiv",
	"glMapGrid1d",
	"glGetCombinerStageParameterfvNV",
	"glSecondaryColor3fEXT",
	"glClear",
	"glSecondaryColor3dEXT",
	"glUniform3iv",
	"glClientActiveTexture",
	"glGetProgramivARB",
	"glRasterPos2iv",
	"glColor3d",
	"glVertexAttrib1d",
	"glAreTexturesResidentEXT",
	"glArrayElementEXT",
	"glGenTexturesEXT",
	"glVertexAttrib1dvARB",
	"glMultiTexCoord3dvARB",
	"glVertex3dv",
	"glVertexAttrib4svNV",
	"glMultiTexCoord4d",
	"glMultiTexCoord3f",
	"glDeleteArraySetsARB",
	"glFlushVertexArrayRangeNV",
	"glBlendEquationSeparateEXT",
	"glVertexAttrib4usvARB",
	"glDetachObjectARB",
	"glBindBuffer",
	"glGetUniformiv",
	"glMultiTexCoord2dSGIS",
	"glNormal3i",
	"glSecondaryColor3uivEXT",
	"glEdgeFlagPointerEXT",
	"glPixelMapusv",
	"glCreateShaderObjectARB",
	"glPointParameterfEXT",
	"glStencilOp",
	"glUniform1fv",
	"glVertexAttrib4NbvARB",
	"glDeleteShader",
	"glColor4d",
	"glProgramEnvParameter4dARB",
	"glBeginConditionalRenderNVX",
	"glDisableClientState",
	"glVertexAttrib1fvNV",
	"glWindowPos3ivARB",
	"glLogicOp",
	"glPolygonMode",
	"glSampleCoverage",
	"glProgramEnvParameter4fARB",
	"glVertex4hNV",
	"glVertexAttrib4sARB",
	"glIsList",
	"glTexCoord3iv",
	"glGetProgramParameterfvNV",
	"glGetVertexAttribPointerv",
	"glMultiDrawArrays",
	"glFogCoordfvEXT",
	"glColor4dv",
	"glVertexAttrib4Nubv",
	"glColor4s",
	"glCreateProgramObjectARB",
	"glVertexAttrib2fvARB",
	"glGetPixelMapuiv",
	"glPointParameteri",
	"glVertexAttribs4ubvNV",
	"glGenBuffers",
	"glTexCoord2sv",
	"glVertexAttrib3svNV",
	"glVertexAttrib4NivARB",
	"glCompileShaderARB",
	"glUniform1f",
	"glColor3hvNV",
	"glEdgeFlag",
	"glVertex3i",
	"glVertexAttrib3fARB",
	"glSecondaryColor3ubv",
	"glVertex2dv",
	"glFogCoordf",
	"glMaterialfv",
	"glVertexAttrib2dARB",
	"glGetVertexAttribPointervNV",
	"glVertexAttribs2hvNV",
	"glFramebufferTexture2DEXT",
	"glActiveTextureARB",
	"glVertexAttrib3dvARB",
	"glMultiTexCoord4fv",
	"glColorPointer",
	"glGenerateMipmapEXT",
	"glFogiv",
	"glMap2f",
	"glColor3b",
	"glColor3sv",
	"glBindArraySetARB",
	"glSecondaryColor3ub",
	"glShadeModel",
	"glGetActiveAttribARB",
	"glScalef",
	"glTexSubImage1D",
	"glLoadName",
	"glLoadTransformEXT",
	"glVertexAttrib4ubvARB",
	"glNormalPointer",
	"glFramebufferRenderbufferEXT",
	"glSampleCoverageARB",
	"glReadBuffer",
	"glCompressedTexSubImage3DARB",
	"glClearDepth",
	"glVertex3iv",
	"glVertexAttrib3svARB",
	"glUniformMatrix4fvARB",
	"glCompressedTexSubImage2DARB",
	"glProgramLocalParameter4fARB",
	"glCombinerStageParameterfvNV",
	"glGetCombinerOutputParameterivNV",
	"glLoadTransposeMatrixdARB",
	"glVertex3s",
	"glCombinerParameterfNV",
	"glGetVertexAttribivARB",
	"glGenQueries",
	"glTexEnvi",
	"glVertexAttrib4d",
	"glVertexAttribs1fvNV",
	"glDisableVertexAttribArrayARB",
	"glVertexAttrib3fARB",
	"glTexCoord4hNV",
	"glCopyTexSubImage3D",
	"glVertexAttrib3dvNV",
	"glColor3fv",
	"glDeleteTextures",
	"glVertex4iv",
	"glGetOcclusionQueryuivNV",
	"glAreProgramsResidentNV",
	"glConvolutionParameterfv",
	"glVertex2d",
	"glWindowPos2sv",
	"glIndexPointerEXT",
	"glLightModelfv",
	"glEvalCoord1d",
	"glGetShaderSource",
	"glVertex2fv",
	"glEndQuery",
	"glNormal3d",
	"glIsEnabled",
	"glMultiTexCoord2i",
	"glGetProgramLocalParameterdvARB",
	"glIndexubv",
	"glVertexAttrib1dvNV",
	"glNormal3bv",
	"glVertexAttrib3fv",
	"glVertexAttrib3fvARB",
	"glTranslatef",
	"glVertexAttribs3svNV",
	"glVertexAttrib2dNV",
	"glGetShaderInfoLog",
	"glRects",
	"glDepthBoundsEXT",
	"glUniform4iv",
	"glIsShader",
	"glColorTable",
	"glPopName",
	"glTexCoord3dv",
	"glGetPointerv",
	"glGenProgramsARB",
	"glBlendEquation",
	"glVertexAttrib4dvARB",
	"glVertexAttrib4NbvARB",
	"glEdgeFlagv",
	"glGetFramebufferAttachmentParameterivEXT",
	"glDisableVertexAttribArrayARB",
	"glIndexs",
	"glWindowPos3f",
	"glIsProgramARB",
	"glGetUniformLocationARB",
	"glColor4f",
	"glGetProgramEnvParameterdvARB",
	"glVertexAttrib2dvARB",
	"glVertexAttrib4Nbv",
	"glIndexdv",
	"glGetCombinerInputParameterivNV",
	"glVertexAttrib1dNV",
	"glColor4i",
	"glGetVertexAttribfvNV",
	"glAddSwapHintRectWIN",
	"glTexCoord3hvNV",
	"glVertexAttrib1sARB",
	"glLoadTransposeMatrixd",
	"glBlendFunc",
	"glEvalCoord2f",
	"glGetTexGendv",
	"glVertexAttrib4dvNV",
	"glMultiTexCoord3sARB",
	"glFeedbackBuffer",
	"glMultiTexCoord3ivARB",
	"glMultiTexCoord1hvNV",
	"glDrawRangeElementsEXT",
	"glRectd",
	"glBufferSubDataARB",
	"glActiveTexture",
	"glWindowPos2dvARB",
	"glVertexAttrib4ubNV",
	"glMultiTexCoord3sSGIS",
	"glEnd",
	"glLightModelf",
	"glRasterPos3s",
	"glVertexAttrib4Niv",
	"glGetConvolutionParameteriv",
	"glVertexAttrib3fvNV",
	"glTexCoord2hNV",
	"glDeleteLists",
	"glVertexAttrib4fARB",
	"glVertexAttribs4fvNV",
	"glSelectTextureSGIS",
	"glLockArraysEXT",
	"glIsFenceNV",
	"glVertexAttrib4uivARB",
	"glIsOcclusionQueryNV",
	"glVertexAttrib1fvARB",
	"glEvalCoord2fv",
	"glDisableVertexAttribArray",
	"glMultiTexCoord4iARB",
	"glGetConvolutionParameterfv",
	"glVertexAttribs4hvNV",
	"glEvalMesh1",
	"glMultiTexCoord3iv",
	"glProgramEnvParameter4fvARB",
	"glUniform4f",
	"glGenOcclusionQueriesNV",
	"glVertex4fv",
	"glGenQueriesARB",
];

pub unsafe fn load_gl_funcs(glfw: &Glfw) {
	for func in FUNCS {
		load_gl_func(func, glfw);
	}
}

unsafe fn load_gl_func(func: &str, glfw: &Glfw) {
	let real_func = glfw.get_proc_address_raw(func);
	if real_func.is_null() {
		panic!("{func} not found by GLFW");
	}
	let real_func = real_func as usize;

	let module = dlopen(std::ptr::null(), RTLD_LAZY);
	let func_str = CString::new(func).unwrap();
	let func_ptr = dlsym(module, func_str.as_ptr());
	if func_ptr.is_null() {
		panic!("{func} not found in main");
	}

	let real_func = real_func as usize;
	let real_func = real_func.to_le_bytes();
	let mut data = Vec::with_capacity(7);
	data.push(0xB8);
	for i in real_func {
		data.push(i);
	}
	data.push(0xFF);
	data.push(0xE0);

	hook::write_memory(func_ptr as *mut (), &data);
}
